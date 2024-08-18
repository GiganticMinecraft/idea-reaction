# idea-reaction

[![CI](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/ci.yaml/badge.svg)](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/ci.yaml)
[![Release idea-reaction-token](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/release.yaml/badge.svg)](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/release.yaml)

アイデア投稿履歴のリアクション (絵文字での反応以外も含めて😉) をサポートするDiscord Bot.

## Features

- アイデア投稿フォームに新しいアイデアが投稿された際に:
    - その投稿履歴に "👍", "👎" のリアクションを付与する
    - その投稿履歴にスレッドを作成する
    - その投稿履歴の Redmine Issue にスレッドへの URL をコメントする

## Usage

- k8s 及び Docker Container 上での動作を想定しています. 整地鯖の k8s で動作するための定義は [seichi_infra](https://github.com/GiganticMinecraft/seichi_infra) で公開しています. ([定義ファイル: `seichi-onp-k8s/manifests/seichi-kubernetes/apps/seichi-minecraft/idea-reaction/idea-reaction.yaml`](https://github.com/GiganticMinecraft/seichi_infra/blob/main/seichi-onp-k8s/manifests/seichi-kubernetes/apps/seichi-minecraft/idea-reaction/idea-reaction.yaml))
- [`compose.yaml`](./compose.yaml) を配置しているリポジトリ内に `.env` ファイルを用意し, 環境変数を設定してください.
- 設定後 `docker compose up -d` で起動できます.

## Development

1. `docker-redmine` 内の `compose.yaml` を実行し, Redmine と postgres を起動します.
2. Redmine にログインし, Issue の作成権限・更新権限を持つユーザを作成し, API Key を取得します.
3. `idea-reaction` の `.env` ファイルに以下の環境変数を追加します.
   - `REDMINE_API_KEY`: Redmine の API Key
   - `REDMINE_URL`: Redmine の URL (Docker 環境の場合 `http://localhost:8080` です.)
4. `cargo run` で idea-reaction を起動します.

## Release (for GiganticMinecraft admin)

- idea-reaction は GitHub App (release-please などのツール) を使った自動リリースには対応していません.
- リリースをするときは `Cargo.toml` の `version` プロパティなどを bump したうえで [Actions](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/release.yaml) にある `Run workflow` から CI を実行すると ghcr.io に Docker Image がリリースされます.
- k8s 本番環境にある idea-reaction を更新する場合は `.../apps/seichi-minecraft/idea-reaction/idea-reaction.yaml` の `spec.template.spec.containers.image` の値を変更してください. 
  - タグ等の情報は [Packages](https://github.com/GiganticMinecraft/idea-reaction/pkgs/container/idea-reaction)
        から確認できます.

## Environment Variables

| Name                | Description                          |
|---------------------|--------------------------------------|
| `DISCORD_API_TOKEN` | Discord Bot のトークン                    |
| `REDMINE_API_KEY`   | Redmine の API Key (コメント機能を有効にする場合のみ) |
| `REDMINE_URL`       | Redmine の URL (コメント機能を有効にする場合のみ)     |
| `TARGET_CHANNEL_ID` | 監視対象チャンネルのID                         |
| `TARGET_WEBHOOK_ID` | 監視対象WebhookのID                       |
| `TARGET_GUILD_ID`   | 監視対象ギルドのID                           |

## Feature Flags

| Feature Flag                 | Description             |
|------------------------------|-------------------------|
| `experiments_thinking_emoji` | 付与されるリアクションに "🤔" を追加する |
