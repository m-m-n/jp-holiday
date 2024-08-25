# 内閣府が提供してる休日情報を取得してRedisに保存する

## 取得元のURL

- [https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv](https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv)

## やってること

- 上記のURLにアクセスし、CSVファイルから日付をkey、休日の説明をvalueとしてRedisに保存する

## 使用例

```
#/bin/bash

HOLIDAY=$(echo -e $(redis-cli get $(date "+%Y/%m/%d")))
if [ -z "$HOLIDAY" ]; then
  echo "祝日じゃないです"
else
  echo "祝日です"
fi
```
