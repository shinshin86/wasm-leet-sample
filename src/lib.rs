use wasm_bindgen::prelude::*;
use rand::Rng;

fn get_leet_list() -> Vec<Vec<String>> {
    let a = vec!["A", "4", "/\\", "@", "/-\\", "^", "aye", "(L", "Д"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let b = vec![
        "B", "I3", "8", "13", "|3", "ß", "P>", "|:", "!3", "(3", "/3", ")3", "|-]", "j3",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let c = vec!["C", "[", "¢", "<", "(", "©"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let d = vec![
        "D", ")", "|)", "(|", "|o", "[)", "I>", "|>", "T)", "I7", "cl", "|}", "|]",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let e = vec!["E", "3", "&", "£", "€", "ë", "[-", "|=-"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let f = vec!["F", "|=", "ƒ", "|#", "ph", "/=", "v"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let g = vec![
        "G", "6", "&", "(_+", "9", "C-", "gee", "(?,", "[,", "{,", "<-", "(.", "₲",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let h = vec![
        "H", "#", "/-/", "[-]", "]-[", ")-(", "(-)", ":-:", "|~|", "|-|", "]~[", "];{", "!-!",
        "1-1", "-/", "I+I", "];-{",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let i = vec!["I", "1", "!", "¡", "|", "eye", "3y3", "][", "]", "/me"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let j = vec!["J", "_|", "_/", "¿", "</", "_]", "(/"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let k = vec!["K", "X", "|<", "|{", "]{", "|X"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let l = vec!["L", "1", "£", "7", "1_", "|", "|_", "el", "[]_", "[_"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let m = vec![
        "M", "|v|", "[V]", "{V}", "\\/V\\", "em", "AA", "|\\/|", "/\\/\\", "(u)", "(V)", "(/)",
        "/|\\", "^^", "/|/|", "//\\", "||\\", "]\\/[",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let n = vec![
        "N", "^/", "|\\|", "/\\/", "[\\]", "<\\>", "en", "[]\\", "//", "[]", "/V", "1V", "И", "^",
        "ท",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let o = vec!["O", "0", "()", "oh", "[]", "p", "<>", "Ø"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let p = vec![
        "P", "|*", "|o", "|º", "|^(o)", "|>", "|\"", "9", "[]D", "|°", "|7",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let q = vec!["Q", "(_,)", "()_", "0_", "<|", "&"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let r = vec![
        "R", "I2", "|`", "|~", "|?", "/2", "|^", "lz", "|9", "2", "12", "®", "[z", "Я", ".-", "|2",
        "|-",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let s = vec!["S", "5", "$", "z", "§", "ehs", "es", "2"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let t = vec!["T", "7", "+", "-|-", "']['", "†", "\\\"|\\"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let u = vec!["U", "(_)", "|_|", "v", "L|", "µ", "บ"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let v = vec!["V", "\\/", "|/", "\\|"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let w = vec![
        "W",
        "\\/\\/",
        "vv",
        "\\N",
        "'//",
        "\\\\'",
        "\\^/",
        "dubya",
        "(n)",
        "\\V/",
        "\\X/",
        "\\|/",
        "\\_|_/",
        "\\_:_/",
        "Ш",
        "uu",
        "2u",
        "\\\\//\\\\//",
        "พ",
        "ω",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let x = vec!["X", "><", "Ж", "];{", "ecks", "×", "?", ")(", "][", "];{"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let y = vec!["Y", "j", "`/", "Ч", "7", "\\|/", "¥"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let z = vec!["Z", "2", "7_", "-/_", "%", ">_", "s", "~/_", "-\\_", "-|_"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    vec![
        a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z,
    ]
}

fn get_leet_index(c: String, leet_index: Vec<String>) -> (bool, usize) {
    for (i, val) in leet_index.iter().enumerate() {
        if c == *val {
            return (true, i);
        }
    }
    (false, 0)
}

fn get_random_num(max: usize) -> usize {
    rand::thread_rng().gen_range(0..max)
}

fn leet(text: String) -> String {
    let leets = get_leet_list();
    let mut leet_index: Vec<String> = Vec::new();
    for (i, _) in leets.iter().enumerate() {
        leet_index.push(leets[i][0].to_string());
    }

    let mut leet_str: String = "".to_string();

    for c in text.chars() {
        let upper_c = c.to_uppercase();
        let result = get_leet_index(upper_c.to_string(), leet_index.clone());
        if result.0 {
            let rnum = get_random_num(leets[result.1].len());
            leet_str = leet_str + &leets[result.1][rnum];
        } else {
            leet_str = leet_str + &c.to_string();
        }
    }

    leet_str
}

#[wasm_bindgen]
pub fn convert_leet (str :String) -> String {
    return leet(str);
}