# GitHub Contributions Checker

## 目的
このプロジェクトは、GitHub の毎日の Contribution 数やヒートマップを確認できるデスクトップアプリケーション (exe) を実現するものです。ユーザーは自分の GitHub アカウントの活動状況を視覚的に把握でき、日々のコミット数や活動パターンが一目で確認できます。

## 使用スタック
- Rust

- egui / eframe

- GraphQL

- GitHub GraphQL API

## 使い方
#### 環境変数ファイルの準備
プロジェクトルートに配置されている env.example ファイルをコピーして、.env というファイル名で保存します。

#### 認証情報の入力
 - GITHUB_TOKEN
GitHub の Personal Access Token を記載します。トークンは GitHub の Developer settings → Personal access tokens から作成できます。
 - GITHUB_USERNAME
自身の GitHub ユーザー名を指定してください。

#### 配置場所
ビルドして生成された exe ファイルと同じディレクトリに .env ファイルを配置します。

#### 実行
exe ファイルを起動すると、GUI 上にあなたの GitHub Contributions 数やヒートマップが表示され、日々の活動を視覚的に確認することができます。

## 詳細・概要
#### 背景
毎日GitHubのContributionsを確認するために、ブラウザを開いて、ブックマークから飛んで、スクロールして･･･というのが少し面倒くさくなったので、Rust(egui)・GraphQLの練習のために作成。


#### 主な機能

 - GitHub GraphQL API を使用した Contribution データの取得

 - egui を利用したリアルタイムな GUI 表示

 - Contributions ヒートマップの作成と描画 (各日の活動数に応じた色で表示)

 - エラーメッセージやログの表示による、ユーザーフィードバックの強化

#### 利点

 - 軽量で高速: Rust のパフォーマンスのおかげで素早いデータ取得と描画が可能

 - シンプルなセットアップ: 認証情報を .env に記載するだけで、簡単に利用可能

 - クロスプラットフォーム対応: 基本は Windows 向けですが、設定次第で他の OS への展開も検討可能
