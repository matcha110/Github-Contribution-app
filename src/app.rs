use eframe::egui;
use reqwest;
use serde::Deserialize;
use std::env;
use std::sync::mpsc;
use std::thread;
use chrono::Local;

// GitHub API 用の構造体群
#[derive(Deserialize)]
struct GraphQLResponse {
    data: Option<GraphQLData>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Deserialize)]
struct GraphQLData {
    user: Option<User>,
}

#[derive(Deserialize)]
struct User {
    #[serde(rename = "contributionsCollection")]
    contributions_collection: ContributionsCollection,
}

#[derive(Deserialize)]
struct ContributionsCollection {
    #[serde(rename = "contributionCalendar")]
    contribution_calendar: ContributionCalendar,
}

#[derive(Deserialize, Clone)]
pub struct ContributionCalendar {
    #[serde(rename = "totalContributions")]
    pub total_contributions: u32,
    pub weeks: Vec<Week>,
}

#[derive(Deserialize, Clone)]
pub struct Week {
    #[serde(rename = "contributionDays")]
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Deserialize, Clone)]
pub struct ContributionDay {
    pub date: String,
    #[serde(rename = "contributionCount")]
    pub contribution_count: u32,
    pub color: String,
}

// 型エイリアス
type FetchResult = Result<(ContributionCalendar, String), String>;

// 環境変数からトークンを取得する関数
fn get_token() -> String {
    env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set in .env")
}
// 環境変数からUserNameを取得する関数
fn get_username() -> String {
    env::var("GITHUB_USERNAME").expect("GITHUB_USERNAME must be set in .env")
}

// TemplateApp の定義
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    label: String,
    value: f32,

    #[serde(skip)]
    contribution_calendar: Option<ContributionCalendar>,
    #[serde(skip)]
    error_message: Option<String>,
    #[serde(skip)]
    fetching: bool,
    #[serde(skip)]
    fetch_receiver: Option<mpsc::Receiver<FetchResult>>,

    #[serde(skip)]
    raw_response_log: Option<String>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            contribution_calendar: None,
            error_message: None,
            fetching: false,
            fetch_receiver: None,
            raw_response_log: None,
        }
    }
}


impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    // GitHub GraphQL API を呼び出してデータ取得
    fn fetch_contributions(&mut self) {
        if self.fetching {
            return;
        }
        self.fetching = true;
        self.error_message = None;
        let (tx, rx) = mpsc::channel();
        self.fetch_receiver = Some(rx);

        let username = get_username();

        let query =
            format!(r#"{{
                user(login: "{}") {{
                    contributionsCollection {{
                        contributionCalendar {{
                            totalContributions
                            weeks {{
                                contributionDays {{
                                    date
                                    contributionCount
                                    color
                                }}
                            }}
                        }}
                    }}
                }}
            }}"#, username);

        thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            let res = client
                .post("https://api.github.com/graphql")
                .header("Authorization", format!("bearer {}", get_token()))
                .header("Content-Type", "application/json")
                .header("User-Agent", "my-rust-app")
                .json(&serde_json::json!({ "query": query }))
                .send();
            match res {
                Err(e) => {
                    let _ = tx.send(Err(format!("Request error: {}", e)));
                }
                Ok(response) => {
                    let response_text = response
                        .text()
                        .unwrap_or_else(|e| format!("Response text error: {}", e));
                    println!("Received raw response: {}", response_text);
                    let json: Result<GraphQLResponse, _> = serde_json::from_str(&response_text);
                    match json {
                        Err(e) => {
                            let _ = tx.send(Err(format!("JSON parse error: {}", e)));
                        }
                        Ok(api_response) => {
                            if let Some(errs) = api_response.errors {
                                let msgs: Vec<String> = errs
                                    .into_iter()
                                    .map(|e| e.message)
                                    .collect();
                                let _ = tx.send(Err(format!("API errors: {:?}", msgs)));
                            } else if let Some(data) = api_response.data {
                                if let Some(user) = data.user {
                                    let cal = user.contributions_collection.contribution_calendar;
                                    let _ = tx.send(Ok((cal, response_text)));
                                } else {
                                    let _ = tx.send(Err("No user data returned".to_string()));
                                }
                            } else {
                                let _ = tx.send(Err("No data returned".to_string()));
                            }
                        }
                    }
                }
            }
        });
    }
}

fn hex_to_color32(hex: &str) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            let r = ((rgb >> 16) & 0xff) as u8;
            let g = ((rgb >> 8) & 0xff) as u8;
            let b = (rgb & 0xff) as u8;
            return egui::Color32::from_rgb(r, g, b);
        }
    }
    egui::Color32::GRAY
}

// eframe::App の実装
impl eframe::App for TemplateApp {
    // eframe::App の save 関数
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    // eframe::App の update 関数
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // バックグラウンド処理からの結果受信
        if let Some(rx) = &self.fetch_receiver {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok((cal, raw_log)) => {
                        self.contribution_calendar = Some(cal);
                        self.error_message = None;
                        self.raw_response_log = Some(raw_log);
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                        self.contribution_calendar = None;
                        self.raw_response_log = None;
                    }
                }
                self.fetching = false;
                self.fetch_receiver = None;
                ctx.request_repaint();
            }
        }

        // トップパネル
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // メニューバー
            egui::menu::bar(ui, |ui| {
                if ui.button("Show Contributions").clicked() {
                    if !self.fetching {
                        self.fetch_contributions();
                    }
                }
            });
        });

        // メインパネル
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("{}'s GitHub Contributions", get_username()));
            if let Some(ref cal) = self.contribution_calendar {
                ui.label(format!("Total Contributions: {}", cal.total_contributions));
            }
            if let Some(ref err) = self.error_message {
                ui.colored_label(egui::Color32::RED, err);
            }

            // if let Some(ref log) = self.raw_response_log {
            //     ui.label("Raw Response Log:");
            //     ui.text_edit_multiline(&mut log.clone());
            // }

            // 今日のコントリビューション状況を表示
            if let Some(ref cal) = self.contribution_calendar {
                let today = Local::now().format("%Y-%m-%d").to_string();
                let mut today_found = false;
                for week in &cal.weeks {
                    for day in &week.contribution_days {
                        if day.date == today {
                            today_found = true;
                            if day.contribution_count > 0 {
                                ui.colored_label(
                                    egui::Color32::GREEN,
                                    format!(
                                        "Contributed today: {} contributions",
                                        day.contribution_count
                                    )
                                );
                            } else {
                                ui.colored_label(egui::Color32::RED, "No contributions today");
                            }
                            break;
                        }
                    }
                    if today_found {
                        break;
                    }
                }
                if !today_found {
                    ui.label("No data available for today");
                }
            }

            ui.separator();

            // Contribution グリッドの描画
            if let Some(ref cal) = self.contribution_calendar {
                let cell_size = 10.0;
                let cell_margin = 2.0;
                let weeks = &cal.weeks;
                let grid_width = (weeks.len() as f32) * (cell_size + cell_margin);
                let grid_height = 7.0 * (cell_size + cell_margin);

                let (rect, _response) = ui.allocate_exact_size(
                    egui::vec2(grid_width, grid_height),
                    egui::Sense::hover()
                );
                let painter = ui.painter_at(rect);

                for (week_index, week) in weeks.iter().enumerate() {
                    for (day_index, day) in week.contribution_days.iter().enumerate() {
                        let x = rect.min.x + (week_index as f32) * (cell_size + cell_margin);
                        let y = rect.min.y + (day_index as f32) * (cell_size + cell_margin);
                        let cell_rect = egui::Rect::from_min_size(
                            egui::pos2(x, y),
                            egui::vec2(cell_size, cell_size)
                        );
                        painter.rect_filled(cell_rect, 0.0, hex_to_color32(&day.color));
                    }
                }
            }
        });

        // フッター
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            linked_to_my_url(ui);
        });
    }
}

// 自分のMypageリンクを表示する関数
fn linked_to_my_url(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Linked by ");
        ui.hyperlink_to("matcha110", "https://github.com/matcha110");
        ui.label(".");
    });
}
