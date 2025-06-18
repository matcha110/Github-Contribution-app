# GitHub Contributions Checker

## Purpose
This project is a desktop application (exe) that allows users to check their daily GitHub Contributions and heatmap. Users can visually grasp the activity status of their GitHub account, seeing daily commit counts and activity patterns at a glance.

## Tech Stack
- Rust

- egui / eframe

- GraphQL

- GitHub GraphQL API

## How to Use
- Prepare the environment variable file  
  Copy the `env.example` file located in the project root directory and save it as `.env`.

- Enter authentication information  
  - `GITHUB_TOKEN`  
    Enter your GitHub Personal Access Token. Tokens can be created from GitHub's Developer settings → Personal access tokens.
  - `GITHUB_USERNAME`  
    Specify your GitHub username.

- File placement  
  Place the `.env` file in the same directory as the built executable (`exe`) file.

- Run  
  When you launch the exe file, your GitHub Contributions and heatmap will be displayed on the GUI, allowing you to visually track your daily activity.

## Details / Overview
- Background  
  Checking GitHub Contributions every day by opening the browser, going through bookmarks, and scrolling became a bit of a hassle—so this app was created as practice for Rust (egui) and GraphQL.

- Main Features  
  - Fetch Contribution data using the GitHub GraphQL API  
  - Real-time GUI display using egui  
  - Render a Contributions heatmap with color-coding based on activity level per day  
  - Show error messages and logs to enhance user feedback  

- Merit  
  - Lightweight and fast: Thanks to Rust's performance, quick data fetching and rendering are possible  
  - Simple setup: Just put the authentication info into the `.env` file to start using it easily  
  - Cross-platform compatible: Although primarily for Windows, deployment to other OSes can be considered with configuration adjustments


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
<<<<<<< Updated upstream
#### 背景
毎日GitHubのContributionsを確認するために、ブラウザを開いて、ブックマークから飛んで、スクロールして･･･というのが少し面倒くさくなったので、Rust(egui)・GraphQLの練習のために作成しました。
=======
- 背景
毎日GitHubのContributionsを確認するために、ブラウザを開いて、ブックマークから飛んで、スクロールして･･･というのが少し面倒くさくなったのとRust(egui)・GraphQLの練習のために作成。
>>>>>>> Stashed changes


#### 主な機能

 - GitHub GraphQL API を使用した Contribution データの取得

 - egui を利用したリアルタイムな GUI 表示

 - Contributions ヒートマップの作成と描画 (各日の活動数に応じた色で表示)

 - エラーメッセージやログの表示による、ユーザーフィードバックの強化

#### 利点

 - 軽量で高速: Rust のパフォーマンスのおかげで素早いデータ取得と描画が可能

 - シンプルなセットアップ: 認証情報を .env に記載するだけで、簡単に利用可能

 - クロスプラットフォーム対応: 基本は Windows 向けですが、設定次第で他の OS への展開も検討可能

#### プレビュー
![image](https://github.com/user-attachments/assets/38d65222-c8cc-4466-a6db-4dd5cdb7b550)

