use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    /// 입력이 구독자 이름에 관한 모든 검증 제약을 만족하면
    /// `SubcriberName` 인스턴스를 반환한다.
    /// 그렇지 않으면 패닉에 빠진다.
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        // `.trim()` 입력 `s`(뒤에 이어지는 화이트 스페이스 등의 문가자 제거된)에
        // 대한 뷰를 반환한다.
        // `.is_empty`는 해당 뷰가 문자를 포함하는지 확인한다.
        let is_empty_or_whitespace = s.trim().is_empty();

        // 자소(grapheme)는 "사용자가 인지할 수 있는 문자"로서 유니코드 표준에 따라
        // 정의된다. `å`는 하나의 자소이지만, 2개의 문자(`a`와 `̊`)로 구성되어 있다.
        //
        // `graphemes`는 입력 `s`안의 자소들에 대한 이터레이터를 반환한다.
        // `true`는 우리가 확장된 자소 정의 셋(권장되는)을 사용하기 원한다는 것을
        // 지정한다.
        let is_too_long = s.graphemes(true).count() > 256;

        // 입력 `s` 안의 모든 문자들에 대해 반복하면서 금지된 배열 안의 문자와 일치하는지
        // 확인한다.
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a̐".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
