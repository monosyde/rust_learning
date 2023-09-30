
    // let url = "https://rapeed.kaiten.ru/api/latest/spaces/43750/boards";
    // let req = reqwest::blocking::get(url).unwrap();
    // let req_print = req.text().unwrap();
    // println!("req = {}", &req_print);
    // serde_json::Value - работает эффективно
    // Но не делает аллокации данных
    // let des: serde_json::Value = serde_json::from_str(&req_print).unwrap();

    // :? - когда не реализоват trait - Display, но реализован trait - Debug
    // println!("json={:?}", &des);