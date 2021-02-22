use snips_nlu_ontology::{BuiltinEntityKind, IntoBuiltinEntityKind, Language};

#[rustfmt::skip::macros(language_support)]
pub fn supported_languages<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [Language] {
    macro_rules! language_support {
        ($(($entity_kind:ident, [$($language:ident),*]),)*) => {
            match entity_kind.into_builtin_kind() {
                $(
                    BuiltinEntityKind::$entity_kind => &[$(Language::$language,)*],
                )*
            }
        }
    }

    language_support!(
        (AmountOfMoney, [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT, RU]),
        (Duration,      [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT, RU]),
        (Number,        [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT, RU]),
        (Ordinal,       [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT]),
        (Temperature,   [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT]),
        (Datetime,      [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT, RU]),
        (Date,          [EN, FR, RU]),
        (Time,          [EN, FR, RU]),
        (DatePeriod,    [EN, FR, RU]),
        (TimePeriod,    [EN, FR, RU]),
        (Percentage,    [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT, RU]),
        (MusicAlbum,    [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT]),
        (MusicArtist,   [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT]),
        (MusicTrack,    [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT]),
        (City,          [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT, RU]),
        (Country,       [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT, RU]),
        (Region,        [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT, RU]),
    )
}
