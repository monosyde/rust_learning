
#### План по проекту

- [x] сделать структуры для карточек в кайтене
- [x] фильтрация по карточкам
- [ ] добавить wasm 
- [ ] красиво выводить карточки в интерфейсе


##### lint
`cargo clippy`


##### docs
`cargo doc --open`

##### format
`cargo fmt --version`
`cargo fmt`
`cargo fmt --all -- --check`

```
serde_json::Value - работает эффективно
Но не делает аллокации данных
let des: serde_json::Value = serde_json::from_str(&req_print).unwrap();

:? - когда не реализоват trait - Display, но реализован trait - Debug
:#? - прети принт для структур
println!("json={:?}", &des);
```