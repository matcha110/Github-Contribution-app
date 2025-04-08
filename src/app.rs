/// アプリケーションの終了時にも状態を保持できるよう、
/// シャットダウン時の状態の永続化を目的として、serdeクレートの
/// Deserialize（逆シリアライズ）およびSerialize（シリアライズ）トレイトを自動導出しています。
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // 新たなフィールドを追加した場合、既存の古いシリアライズデータを逆シリアライズする際に、
/// 追加されたフィールドにはあらかじめ定義したデフォルト値を設定するようにします。
pub struct TemplateApp {
    // --- アプリ内で使用するデータの例 --- 
    /// ユーザーに表示するテキストや入力された文字列を保持するフィールドです。
    label: String,

    #[serde(skip)] // この属性を付与することで、
/// このフィールドはシリアライズ（状態の保存）の対象から除外されます。
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // --- デフォルト値の設定 --- 
            /// labelフィールドには初期値として「Hello World!」という文字列を設定します。
            label: "Hello World!".to_owned(),
            /// valueフィールドにはデフォルト値として2.7を設定しています。
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// アプリケーションの最初のフレームが描画される前に一度だけ呼び出される初期化関数です。
    ///
    /// この関数では、以下の処理を行います:
    /// - eguiの表示スタイルやフォントのカスタマイズ（必要に応じて`cc.egui_ctx.set_visuals`や`cc.egui_ctx.set_fonts`を利用可能）
    /// - 以前保存された状態が存在する場合は、それを読み込み、存在しない場合はデフォルトの値を使用する
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // eguiの外観やフォントの設定はここでカスタマイズ可能です。
        // 例: cc.egui_ctx.set_visuals(custom_visuals);
        //     cc.egui_ctx.set_fonts(custom_fonts);

        // --- 保存されたアプリケーション状態の読み込み ---
        // 以前に保存された状態があれば、それをストレージから取得します。
        // ※ この機能を利用するには、Cargo.tomlで`persistence`機能を有効にしておく必要があります。
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        // もし保存された状態が存在しなければ、デフォルトの状態を返します。
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// フレームワークがシャットダウン前に状態を保存するために呼び出す関数です。
    /// この関数内で、現在のアプリケーション状態をストレージに保存します。
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// UIの再描画が要求されるたびに呼び出される関数です。
    /// この処理は1秒間に多数回呼び出される可能性があり、ユーザーの入力や画面の更新に応じて動的に描画を行います。
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- UIウィジェットの配置 --- 
        // ウィジェット（ボタン、テキストフィールド、スライダーなどのUI部品）を
        // SidePanel、TopBottomPanel、CentralPanel、Window、Areaなどのレイアウトコンテナ内に配置してください。
        // 参考例や詳細については、https://emilk.github.io/egui を参照してください。

        // --- 上部パネルの描画 --- 
        // 上部パネルはメニューバーなどの主要なナビゲーション要素を配置するのに適しています。
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // --- メニューバーの配置 ---
            egui::menu::bar(ui, |ui| {
                // 注意: ウェブプラットフォーム（wasm32ターゲット）では「File→Quit」メニューは利用不可です。
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        // 「Quit」ボタンがクリックされた場合、ビューを閉じるようコマンドを送信します。
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    // ボタン配置後に16.0のスペースを追加し、レイアウトを調整します。
                    ui.add_space(16.0);
                }

                // グローバルテーマ（明るい/暗いテーマなど）の切替ボタンをレイアウトに追加します。
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // --- 中央パネルの描画 ---
        // 中央パネルは、上部やサイドのパネルを除いた残りの画面領域です。
        egui::CentralPanel::default().show(ctx, |ui| {
            // アプリケーションのタイトルまたは説明文を見出しとして表示します。
            ui.heading("eframe template");

            // --- 入力エリアの配置 ---
            // ユーザーがテキストを入力できるよう、水平レイアウトでラベルとテキスト入力欄を配置します。
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            // --- スライダーウィジェットの配置 ---
            // 数値の入力や調整を可能にするスライダーを追加し、ラベル「value」を表示します。
            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // 「Increment」ボタンがクリックされた場合、valueの値を1.0だけ増加させます。
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            // --- セクションの区切り線 ---
            ui.separator();

            // GitHub上のソースコードへのリンクを追加します。
            // egui::github_link_file!マクロは、指定したURLとテキストをリンクとして表示します。
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            // --- 下部レイアウトの設定 ---
            // レイアウトを下から上に積み上げる形式に変更し、
            // クレジット表示やデバッグビルドに関する警告文を配置します。
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                // egui及びeframeのクレジット情報を表示する関数を呼び出します。
                powered_by_egui_and_eframe(ui);
                // デバッグビルド時に注意喚起の警告文を表示します。
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

/// eguiとeframeのクレジット（帰属表示）情報をUI上に表示するための関数です。
fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        // アイテム間の水平スペースをゼロに設定し、各要素が連続して表示されるよう調整します。
        ui.spacing_mut().item_spacing.x = 0.0;
        // 「Powered by 」というテキストを表示
        ui.label("Powered by ");
        // egui公式リポジトリへのハイパーリンクを追加
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        // 「 and 」というテキストを表示
        ui.label(" and ");
        // eframe公式リポジトリへのハイパーリンクを追加
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        // 文末のピリオドを表示して文を締めくくります。
        ui.label(".");
    });
}
