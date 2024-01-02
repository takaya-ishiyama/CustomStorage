
## rule
- setterを書いてはならない
- getterは必要最低限にする
- getterで取得した値でロジックを書かない
    - https://zenn.dev/tattu/articles/ee3849159c0888#setter%2Fgetter%E3%81%8C%E4%B8%8D%E8%A6%81%E3%81%AA%E7%90%86%E7%94%B1


## データベース操作
database/pg-dataにマウントされているからデータベースを完全削除するときは消す
sqlx migrate run
sqlx database drop

# memo
cargo fmt でフォーマットの修正が出来る
cargo clippy --fix を使うと自動で直してくれる。