# idea-reaction

アイデア投稿履歴のリアクションを制限する Discord Bot.

## Introduction

- Docker Container 上での動作を想定しています.
- `compose.yaml` を配置しているリポジトリ内に `.env` ファイルを用意し, 環境変数を設定してください.
  - Docker Compose v1 (非推奨) での動作は想定していません.
- 設定後 `docker compose up -d` で起動できます.

## Environment Variables

| Name | Description | Default |
| `DISCORD_TOKEN` | Discord Bot のトークン | - |

## Configuration

- idea-reaction は `config.yaml` によって設定が変更可能です.

```yaml
reaction:
  emoji: ["👍", "👎"] # リアクションに使用する絵文字を文字列で指定する
```
