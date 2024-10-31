use std::{collections::HashMap, error::Error, fs};
use reqwest::blocking::get;
use serde_json::Value;
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}
pub struct App {
    pub title: String,
    pub amount_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub amount: f64,                 // 用户输入的人民币金额
    pub target_currency: String,     // 目标货币
    pub converted_amount: String, // 转换后的金额
    pub exchange_rates: HashMap<String, f64>, // 存储汇率
    pub selected_index: usize, // 新增选中的索引
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            title: String::from("小工具"),
            amount_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            amount: 0.0,
            target_currency: String::new(),
            converted_amount: String::new(),
            exchange_rates: HashMap::new(), // 初始化汇率 HashMap
            selected_index: 1,
        };

        // 在应用启动时获取汇率
        match app.fetch_exchange_rate() {
            Ok(rates) => {
                app.exchange_rates = rates; // 将获取到的汇率存入 app 的 exchange_rates
            }
            Err(err) => {
                eprintln!("获取汇率时出错: {}", err);
            }
        }

        app
    }

    pub fn fetch_exchange_rate(&mut self) -> Result<HashMap<String, f64>, Box<dyn Error>> {
        let file_path = "resources/exchange_rates.json";
        let supported_currencies = vec!["USD", "JPY", "AUD", "ARS", "INR", "GBP", "TRY"];

        // 如果文件存在，则读取文件
        if let Ok(contents) = fs::read_to_string(file_path) {
            let rates: Value = serde_json::from_str(&contents)?;
            return self.extract_rates(&rates, &supported_currencies);
        }

        // 如果文件不存在，则从 API 获取汇率
        let url = "https://api.exchangerate-api.com/v4/latest/CNY";
        let response: Value = get(url)?.json()?;

        // 将获取到的汇率保存到文件
        fs::write(file_path, response.to_string())?;

        self.extract_rates(&response, &supported_currencies) // 返回从 API 获取的汇率
    }

    // 提取汇率的方法
    fn extract_rates(&self, rates: &Value, supported_currencies: &[&str]) -> Result<HashMap<String, f64>, Box<dyn Error>> {
        let mut exchange_rates = HashMap::new();
        for currency in supported_currencies {
            if let Some(rate) = rates["rates"][currency].as_f64() {
                exchange_rates.insert(currency.to_string(), rate);
            }
        }
        Ok(exchange_rates)
    }

    pub fn convert_currency(&mut self) -> Result<(), Box<dyn Error>> {
        // 清空转换结果
        self.converted_amount.clear();

        // 遍历所有支持的货币，计算并显示转换结果
        for (currency, &rate) in &self.exchange_rates {
            let converted = self.amount * rate;
            // 将每个国家的结果格式化并追加到 converted_amount 中
            self.converted_amount.push_str(&format!("{}: {:.2}\n", currency, converted));
        }

        Ok(())
    }
}
