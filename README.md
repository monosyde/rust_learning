
#### План по проекту

- [x] сделать структуры для карточек в кайтене
- [x] фильтрация по карточкам
- [x] добавить форматинг
- [x] добавить простой тест
- [x] пройтись линтером
- [ ] добавить wasm
- [ ] красиво выводить карточки в интерфейсе
- [ ] попробовать сгенерировать документацию
- [ ] пописать в функциональном стиле


##### wasm-pack build
`cd wasm`
`wasm-pack build --release --target web`

##### build new library
`cargo new name_lib`
`cargo new --lib name_lib`

##### lint
`cargo clippy`
`cargo clippy -- -D warnings`

##### test
`cargo test`

##### docs
`cargo doc --open`

##### format
`cargo fmt --version`
`cargo fmt`
`cargo fmt --all -- --check`

##### pre-commit
`pre-commit --version`
`pre-commit install`
`pre-commit run --all-files`
