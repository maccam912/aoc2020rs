enum ProducerType {
    Literal(String),
    And(Vec<ProducerType>),
    Or(Vec<ProducerType>),
}

struct Producer {
    name: String,
    producer: ProducerType,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case() {
        assert_eq!(1, 1);
    }
}