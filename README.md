# idea-reaction

[![Build idea-reaction](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/build.yaml/badge.svg)](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/build.yaml)
[![clippy](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/clippy.yaml/badge.svg)](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/clippy.yaml)
[![rustfmt](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/fmt.yaml/badge.svg)](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/fmt.yaml)
[![Release idea-reaction-token](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/release.yaml/badge.svg)](https://github.com/GiganticMinecraft/idea-reaction/actions/workflows/release.yaml)

アイデア投稿履歴のリアクションを制限する Discord Bot.

## Introduction

- Docker Container 上での動作を想定しています.
- [`compose.yaml`](./compose.yaml) を配置しているリポジトリ内に `.env` ファイルを用意し, 環境変数を設定してください.
  - Docker Compose v1 (非推奨) での動作は想定していません.
- 設定後 `docker compose up -d` で起動できます.

## Environment Variables

| Name | Description |
| ---- | ---- |
| `DISCORD_TOKEN` | Discord Bot のトークン |
| `TARGET_CHANNEL_ID` | 監視対象チャンネルのID |
| `TARGET_WEBHOOK_ID` | 監視対象WebhookのID |
| `TARGET_GUILD_ID` | 監視対象ギルドのID |

## Configuration

- idea-reaction は `config.yaml` によって設定が変更可能です.

```yaml
reaction: ["👍", "👎"] # リアクションに使用する絵文字を文字列の配列(Vec)で指定する
```
