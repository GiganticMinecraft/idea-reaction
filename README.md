# idea-reaction

[![Build idea-reaction](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/build.yaml/badge.svg)](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/build.yaml)
[![clippy](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/clippy.yaml/badge.svg)](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/clippy.yaml)
[![rustfmt](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/fmt.yaml/badge.svg)](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/fmt.yaml)
[![Release idea-reaction-token](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/release.yaml/badge.svg)](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/release.yaml)

アイデア投稿履歴のリアクションを制限する Discord Bot.

## Introduction

- k8s 及び Docker Container 上での動作を想定しています.
  - 整地鯖の k8s で動作するための定義は [seichi_infra](https://github.com/GiganticMinecraft/seichi_infra) で公開しています. ([定義ファイル: `seichi-onp-k8s/manifests/seichi-kubernetes/apps/seichi-minecraft/idea-reaction/idea-reaction.yaml`](https://github.com/GiganticMinecraft/seichi_infra/blob/main/seichi-onp-k8s/manifests/seichi-kubernetes/apps/seichi-minecraft/idea-reaction/idea-reaction.yaml))
- [`compose.yaml`](./compose.yaml) を配置しているリポジトリ内に `.env` ファイルを用意し, 環境変数を設定してください.
  - Docker Compose v1 (非推奨) での動作は想定していません.
- 設定後 `docker compose up -d` で起動できます.

## Release (for GiganticMinecraft admin)

- idea-reaction は GitHub App (release-please などのツール) を使った自動リリースには対応していません.
- リリースをするときは `Cargo.toml` の `version` プロパティなどを bump したうえで [Actions](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/release.yaml) にある `Run workflow` から CI を実行すると ghcr.io に Docker Image がリリースされます.
- k8s 本番環境にある idea-reaction を更新する場合は `.../apps/seichi-minecraft/idea-reaction/idea-reaction.yaml` の `spec.template.spec.containers.image` の値を変更してください.
  - タグ等の情報は [Packages](https://github.com/GiganticMinecraft/idea-reaction/pkgs/container/idea-reaction) から確認できます.

## Environment Variables

| Name | Description |
| ---- | ---- |
| `DISCORD_API_TOKEN` | Discord Bot のトークン |
| `TARGET_CHANNEL_ID` | 監視対象チャンネルのID |
| `TARGET_WEBHOOK_ID` | 監視対象WebhookのID |
| `TARGET_GUILD_ID` | 監視対象ギルドのID |
