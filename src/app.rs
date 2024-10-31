use std::{collections::HashMap, error::Error};
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
        };

        // 在应用启动时获取汇率
        let supported_currencies = vec!["USD", "JPY", "AUD", "ARS", "INR", "GBP", "TRY"];
        for currency in supported_currencies {
            if let Ok(rate) = app.fetch_exchange_rate(currency) {
                app.exchange_rates.insert(currency.to_string(), rate);
            }
        }

        app
    }

    pub fn fetch_exchange_rate(&self, target_currency: &str) -> Result<f64, Box<dyn Error>> {
        // 汇率 API 的 URL
        let url = "https://api.exchangerate-api.com/v4/latest/CNY".to_string(); // 假设以人民币为基准
        let response: Value = get(&url)?.json()?;

        // 查找目标货币的汇率
        let supported_currencies = vec!["USD", "JPY", "AUD", "ARS", "INR", "GBP", "TRY"];
        if supported_currencies.contains(&target_currency) {
            if let Some(rate) = response["rates"][target_currency].as_f64() {
                Ok(rate)
            } else {
                Err("不支持的货币".into())
            }
        } else {
            Err("不支持的货币".into())
        }
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
