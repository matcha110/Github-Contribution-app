#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // リリースビルド時（debug_assertionsが無効の場合）、Windowsでコンソールウィンドウを表示しないようにします。

// --- ネイティブ環境向けのコンパイル設定 ---
// （wasm32以外のターゲット、つまりネイティブアプリとしてコンパイルする場合に使用されます）
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    // env_loggerを初期化し、RUST_LOG環境変数（例: RUST_LOG=debug）に基づき、stderrへログ出力を行います。
    env_logger::init();

    // ネイティブ環境用のオプションを設定します。
    // ここではウィンドウのサイズや最小サイズ、アイコンなどの設定を行っています。
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // 初期のウィンドウ内側のサイズを400x300ピクセルに設定
            .with_inner_size([400.0, 300.0])
            // ウィンドウの最小内側サイズを300x220ピクセルに設定
            .with_min_inner_size([300.0, 220.0])
            // アプリケーションウィンドウに表示するアイコンの設定（任意）
            .with_icon(
                // NOTE: アイコンの追加は任意です。下記ではPNG形式のバイト列からアイコン情報を作成しています。
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("アイコンのロードに失敗しました"),
            ),
        // その他の設定はデフォルト値を使用
        ..Default::default()
    };

    // ネイティブ環境でeFrameアプリケーションを実行します。
    // 第1引数はウィンドウタイトル、第2引数は先ほど設定したネイティブオプション、
    // 第3引数はアプリケーションインスタンスを生成するクロージャを指定しています。
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Ok(Box::new(eframe_template::TemplateApp::new(cc)))),
    )
}

// --- Web環境向けのコンパイル設定 ---
// （wasm32ターゲット、つまりWebAssemblyとしてコンパイルする場合に使用されます）
// trunk等を利用してWeb用アプリとしてビルドする際に有効になります。
#[cfg(target_arch = "wasm32")]
fn main() {
    // wasm_bindgenのJsCastトレイトをインポート（型変換のため）
    use eframe::wasm_bindgen::JsCast as _;

    // Web向けのロガーを初期化し、logメッセージをブラウザのconsole.log等に出力できるようにします。
    // ログレベルはデバッグレベルに設定されています。
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    // Web向けのオプションをデフォルト値で生成します。
    let web_options = eframe::WebOptions::default();

    // 非同期処理をローカルのタスクとして実行します。
    wasm_bindgen_futures::spawn_local(async {
        // Web APIを介してウィンドウオブジェクトを取得し、ドキュメントオブジェクトを参照します。
        let document = web_sys::window()
            .expect("ウィンドウが見つかりません")
            .document()
            .expect("ドキュメントが見つかりません");

        // HTML内で指定されたID ("the_canvas_id") を持つキャンバス要素を取得し、
        // HtmlCanvasElement型へのダウンキャストを行います。
        // このキャンバスはeframeアプリの描画対象になります。
        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("the_canvas_idを持つ要素が見つかりません")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_idはHtmlCanvasElementではありません");

        // WebRunnerを用いて、Web向けにeframeアプリケーションをスタートさせます。
        // スタート処理は非同期で行われ、結果（成功またはエラー）を返します。
        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(eframe_template::TemplateApp::new(cc)))),
            )
            .await;

        // --- ローディング表示の制御 ---
        // ページ上にローディング中のテキストやスピナーがある場合、スタート処理の結果に応じて
        // これらの要素を削除またはエラー表示へ変更します。
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    // アプリケーションの起動が成功した場合、ローディング中のテキストを削除します。
                    loading_text.remove();
                }
                Err(e) => {
                    // 起動に失敗した場合、ローディング要素の内容をエラーメッセージに書き換え、開発者用コンソールに詳細を表示します。
                    loading_text.set_inner_html(
                        "<p> アプリがクラッシュしました。詳細は開発者コンソールをご覧ください。 </p>",
                    );
                    panic!("eframeの起動に失敗しました: {e:?}");
                }
            }
        }
    });
}
