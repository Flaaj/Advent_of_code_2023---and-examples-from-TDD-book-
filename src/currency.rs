#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Currency {
    Dollar,
    Franc,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Money {
    currency: Currency,
    amount: f64,
}

impl Money {
    pub fn new(currency: Currency, amount: f64) -> Self {
        Self { currency, amount }
    }

    pub fn times(self, multiplier: f64) -> Self {
        Self::new(self.currency, self.amount * multiplier)
    }

    pub fn add(self, other: Self) -> AddMoneyExpression {
        AddMoneyExpression {
            ingredients: vec![self, other],
        }
    }

    pub fn equals(self, other: Self) -> bool {
        self.amount == other.amount && self.currency == other.currency
    }

    pub fn get_currency(self) -> Currency {
        self.currency
    }

    pub fn round(self) -> Self {
        Self::new(self.currency, (self.amount * 1e3).round() / 1e3)
    }
}

pub struct AddMoneyExpression {
    ingredients: Vec<Money>,
    // TODO: Maybe implement an "add" function that adds another ingredient to the expression
}

impl AddMoneyExpression {
    pub fn evaluate(
        self,
        bank: &Bank,
        target_currency: Currency,
    ) -> Result<Money, EvaluateAddMoneyExpressionError> {
        let mut sum = 0.0;
        for ingredient in self.ingredients {
            match bank.get_exchange_rate(ingredient.currency, target_currency) {
                Some(exchange_rate) => sum += ingredient.amount / exchange_rate,
                None => Err(EvaluateAddMoneyExpressionError::ExchangeRateNotFound)?,
            }
        }
        Ok(Money::new(target_currency, sum))
    }
}

#[derive(Copy, Clone)]
struct ExchangeRate {
    from: Currency,
    to: Currency,
    value: f64,
}

pub struct Bank {
    exchange_rates: Vec<ExchangeRate>,
}

#[derive(Debug)]
pub enum EvaluateAddMoneyExpressionError {
    ExchangeRateNotFound,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            exchange_rates: Vec::new(),
        }
    }

    pub fn add_exchange_rate(&mut self, from: Currency, to: Currency, value: f64) {
        self.exchange_rates.push(ExchangeRate { from, to, value });
    }

    fn get_exchange_rate(&self, from: Currency, to: Currency) -> Option<f64> {
        if from == to {
            return Some(1.0);
        }
        let exchange_rate = self
            .exchange_rates
            .iter()
            .find(|er| er.from == from && er.to == to);
        match exchange_rate {
            Some(exchange_rate) => Some(exchange_rate.value),
            None => None,
        }
    }

    pub fn evaluate(
        self,
        expression: AddMoneyExpression,
        target_currency: Currency,
    ) -> Result<Money, EvaluateAddMoneyExpressionError> {
        expression.evaluate(&self, target_currency)
    }
}

#[derive(Debug, PartialEq)]
struct Record {
    instrument: String,
    shares: i32,
    price: Money,
    total: Money,
}

impl Record {
    pub fn new(instrument: String, shares: i32, price: f64, currency: Currency) -> Self {
        let price = Money::new(currency, price);
        Self {
            instrument,
            shares,
            price,
            total: price.times(shares as f64),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Report {
    records: Vec<Record>,
    sum: Money,
}

#[derive(Debug, PartialEq)]
struct Reporter {
    records: Vec<Record>,
}

#[derive(Debug)]
enum ReportGenerationError {
    NotAllNeededExchangeRatesAdded,
}

impl Reporter {
    pub fn new() -> Self {
        Self { records: vec![] }
    }

    pub fn add_record(&mut self, instrument: String, shares: i32, price: f64, currency: Currency) {
        self.records
            .push(Record::new(instrument, shares, price, currency))
    }

    pub fn generate(
        self,
        bank: Bank,
        target_currency: Currency,
    ) -> Result<Report, ReportGenerationError> {
        let mut sum = Money::new(target_currency, 0.0);

        for record in &self.records {
            let expression = sum.add(record.total);
            let evaluated = expression.evaluate(&bank, target_currency);
            match evaluated {
                Ok(evaluated) => sum = evaluated,
                Err(_) => {
                    Err(ReportGenerationError::NotAllNeededExchangeRatesAdded)?;
                }
            }
        }

        sum = sum.round();

        Ok(Report {
            records: self.records,
            sum,
        })
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::currency::{Bank, Currency, Money, Record, Report, Reporter};

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(11.0, 11.0)]
    fn money_with_the_same_currency_and_amount_are_equal(
        #[case] amount_one: f64,
        #[case] amount_two: f64,
    ) {
        let money_one = Money::new(Currency::Dollar, amount_one);
        let money_two = Money::new(Currency::Dollar, amount_two);

        let is_equal = money_one.equals(money_two);

        assert!(is_equal);
    }

    #[rstest]
    #[case(1.0, 2.0)]
    #[case(11.0, 22.0)]
    fn money_with_the_same_currency_and_different_amount_are_not_equal(
        #[case] amount_one: f64,
        #[case] amount_two: f64,
    ) {
        let sut = Money::new(Currency::Dollar, amount_one);
        let other = Money::new(Currency::Dollar, amount_two);

        let is_equal = sut.equals(other);

        assert!(!is_equal);
    }

    #[test]
    fn money_with_different_currency_and_the_same_amount_are_not_equal() {
        let dollar = Money::new(Currency::Dollar, 10.0);
        let franc = Money::new(Currency::Franc, 10.0);

        let is_equal = dollar.equals(franc);

        assert!(!is_equal);
    }

    #[rstest]
    #[case(5.0, 2.0, 10.0)]
    #[case(5.0, 3.0, 15.0)]
    fn multiplies_money_amount(
        #[case] amount: f64,
        #[case] multiplier: f64,
        #[case] expected: f64,
    ) {
        let sut = Money::new(Currency::Dollar, amount);
        let expected = Money::new(Currency::Dollar, expected);

        let result = sut.times(multiplier);

        let is_equal = result.equals(expected);
        assert!(is_equal);
    }

    #[test]
    fn adds_the_same_currencies() {
        let money_one = Money::new(Currency::Dollar, 10.0);
        let money_two = Money::new(Currency::Dollar, 15.0);
        let bank = Bank::new();

        let expression = money_one.add(money_two);
        let sum = bank.evaluate(expression, Currency::Dollar);

        let is_equal = sum.unwrap().equals(Money::new(Currency::Dollar, 25.0));
        assert!(is_equal)
    }

    #[test]
    fn errors_when_exchanging_currencies_without_providing_exchange_rate() {
        let money_one = Money::new(Currency::Dollar, 10.0);
        let money_two = Money::new(Currency::Franc, 10.0);
        let bank = Bank::new();

        let expression = money_one.add(money_two);
        let reduced = bank.evaluate(expression, Currency::Dollar);

        assert!(reduced.is_err());
    }

    #[test]
    fn adds_different_currencies() {
        let money_one = Money::new(Currency::Dollar, 10.0);
        let money_two = Money::new(Currency::Franc, 15.0);
        let mut bank = Bank::new();
        bank.add_exchange_rate(Currency::Franc, Currency::Dollar, 2.0);

        let expression = money_one.add(money_two);
        let reduced = bank.evaluate(expression, Currency::Dollar);

        assert!(reduced.unwrap().equals(Money::new(Currency::Dollar, 17.5)));
    }

    #[test]
    fn generates_report_for_instrument_shares_in_one_currency() {
        let mut reporter = Reporter::new();
        reporter.add_record(String::from("IBM"), 1000, 25.0, Currency::Dollar);
        reporter.add_record(String::from("GE"), 400, 100.0, Currency::Dollar);
        let bank = Bank::new();

        let report = reporter.generate(bank, Currency::Dollar).unwrap();

        assert!(report.records.contains(&Record {
            instrument: String::from("IBM"),
            shares: 1000,
            price: Money::new(Currency::Dollar, 25.0),
            total: Money::new(Currency::Dollar, 25000.0)
        }));
        assert!(report.records.contains(&Record {
            instrument: String::from("GE"),
            shares: 400,
            price: Money::new(Currency::Dollar, 100.0),
            total: Money::new(Currency::Dollar, 40000.0)
        }));
        assert!(report.sum.equals(Money::new(Currency::Dollar, 65000.0)))
    }

    #[test]
    fn generates_report_for_instrument_shares_in_different_currencies() {
        let mut reporter = Reporter::new();
        reporter.add_record(String::from("IBM"), 1000, 25.0, Currency::Dollar);
        reporter.add_record(String::from("GE"), 800, 150.0, Currency::Franc);
        let mut bank = Bank::new();
        bank.add_exchange_rate(Currency::Franc, Currency::Dollar, 1.5);

        let report = reporter.generate(bank, Currency::Dollar).unwrap();

        assert!(report.sum.equals(Money::new(Currency::Dollar, 105000.0)))
    }

    #[test]
    fn generates_report_for_instrument_shares_in_different_currencies_case_2() {
        let mut reporter = Reporter::new();
        reporter.add_record(String::from("IBM"), 100, 200.0, Currency::Dollar);
        reporter.add_record(String::from("GE"), 800, 150.0, Currency::Franc);
        let mut bank = Bank::new();
        bank.add_exchange_rate(Currency::Dollar, Currency::Franc, 0.66666666);

        let report = reporter.generate(bank, Currency::Franc).unwrap();

        assert!(report.sum.equals(Money::new(Currency::Franc, 150000.0)));
    }
}
