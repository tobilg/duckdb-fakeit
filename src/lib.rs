use duckdb_loadable_macros::duckdb_entrypoint_c_api_v2;
use duckdb_neo::{
    connection::Context,
    data_chunk::VectorCollection,
    enums::{FunctionProperty, FunctionPropertyStability},
    extension::Extension,
    scalar::{ScalarCallbacks, ScalarFunctionBuilder},
    signature::{FunctionDocumentationBuilder, FunctionParameter, SignatureBuilder},
    vector::{Unknown, Vector},
    DuckDBType, Result,
};

fn function_documentation(
    name: &str,
    category: &str,
    description: &str,
    example_arguments: &str,
) -> FunctionDocumentationBuilder {
    FunctionDocumentationBuilder::new()
        .description(description)
        .add_example("", format!("{name}({example_arguments})"))
        .add_category("fakeit")
        .add_category(category)
}

// Macro to create a zero-argument VARCHAR scalar function struct.
macro_rules! varchar_scalar {
    ($name:ident, $func:expr) => {
        struct $name;

        impl ScalarCallbacks for $name {
            type BindData = ();
            type InitData = ();

            fn exec(
                &self,
                _bind_data: Option<&Self::BindData>,
                _init_data: Option<&mut Self::InitData>,
                _context: &Context,
                input: &mut VectorCollection,
                output: Vector<'_, Unknown>,
            ) -> Result<()> {
                let len = input.row_count()?;
                let mut output = output.cast::<String>()?;
                output.set_size(len)?;
                let mut writer = output.writer()?;

                for i in 0..len {
                    let result: String = $func();
                    writer.set(i, Some(result.as_str()))?;
                }
                Ok(())
            }
        }

        impl $name {
            fn register(
                extension: &Extension,
                name: &str,
                category: &str,
                description: &str,
            ) -> Result<()> {
                ScalarFunctionBuilder::new(
                    name,
                    SignatureBuilder::new()
                        .return_type(String::logical_type(extension)?)
                        .documentation(function_documentation(name, category, description, "")),
                    Self,
                )
                .set_property(FunctionProperty::Stability(
                    FunctionPropertyStability::Volatile,
                ))
                .register(extension)
            }
        }
    };
}

// Macro for zero-argument BIGINT, DOUBLE, and BOOLEAN scalar functions.
macro_rules! primitive_scalar {
    ($name:ident, $type:ty, $func:expr) => {
        struct $name;

        impl ScalarCallbacks for $name {
            type BindData = ();
            type InitData = ();

            fn exec(
                &self,
                _bind_data: Option<&Self::BindData>,
                _init_data: Option<&mut Self::InitData>,
                _context: &Context,
                input: &mut VectorCollection,
                output: Vector<'_, Unknown>,
            ) -> Result<()> {
                let len = input.row_count()?;
                let mut output = output.cast::<$type>()?;
                output.set_size(len)?;
                let mut writer = output.writer()?;

                for i in 0..len {
                    writer.set(i, Some($func()))?;
                }
                Ok(())
            }
        }

        impl $name {
            fn register(
                extension: &Extension,
                name: &str,
                category: &str,
                description: &str,
            ) -> Result<()> {
                ScalarFunctionBuilder::new(
                    name,
                    SignatureBuilder::new()
                        .return_type(<$type>::logical_type(extension)?)
                        .documentation(function_documentation(name, category, description, "")),
                    Self,
                )
                .set_property(FunctionProperty::Stability(
                    FunctionPropertyStability::Volatile,
                ))
                .register(extension)
            }
        }
    };
}

// Macro to create a two-DOUBLE-argument DOUBLE scalar function struct.
macro_rules! double_double_scalar {
    ($name:ident, $func:expr) => {
        struct $name;

        impl ScalarCallbacks for $name {
            type BindData = ();
            type InitData = ();

            fn exec(
                &self,
                _bind_data: Option<&Self::BindData>,
                _init_data: Option<&mut Self::InitData>,
                _context: &Context,
                input: &mut VectorCollection,
                output: Vector<'_, Unknown>,
            ) -> Result<()> {
                let len = input.row_count()?;
                let min = input.get_vector_at::<f64>(0)?.reader()?;
                let max = input.get_vector_at::<f64>(1)?.reader()?;
                let mut output = output.cast::<f64>()?;
                output.set_size(len)?;
                let mut writer = output.writer()?;

                for i in 0..len {
                    let value = match (min.get(i)?, max.get(i)?) {
                        (Some(min), Some(max)) => Some(($func)(*min, *max)),
                        _ => None,
                    };
                    writer.set(i, value)?;
                }
                Ok(())
            }
        }

        impl $name {
            fn register(
                extension: &Extension,
                name: &str,
                category: &str,
                description: &str,
            ) -> Result<()> {
                ScalarFunctionBuilder::new(
                    name,
                    SignatureBuilder::new()
                        .param(FunctionParameter::new::<f64>("min"))
                        .param(FunctionParameter::new::<f64>("max"))
                        .return_type(f64::logical_type(extension)?)
                        .documentation(function_documentation(
                            name,
                            category,
                            description,
                            "-10.0, 10.0",
                        )),
                    Self,
                )
                .set_property(FunctionProperty::Stability(
                    FunctionPropertyStability::Volatile,
                ))
                .register(extension)
            }
        }
    };
}

// Define all the scalar functions using macros
// Address functions
varchar_scalar!(AddressStreet, fakeit::address::street);
varchar_scalar!(AddressCity, fakeit::address::city);
varchar_scalar!(AddressState, fakeit::address::state);
varchar_scalar!(AddressZip, fakeit::address::zip);
varchar_scalar!(AddressCountry, fakeit::address::country);
varchar_scalar!(AddressStreetNumber, fakeit::address::street_number);
varchar_scalar!(AddressStreetPrefix, fakeit::address::street_prefix);
varchar_scalar!(AddressStreetName, fakeit::address::street_name);
varchar_scalar!(AddressStreetSuffix, fakeit::address::street_suffix);
varchar_scalar!(AddressStateAbr, fakeit::address::state_abr);
varchar_scalar!(AddressCountryAbr, fakeit::address::country_abr);

// Name functions
varchar_scalar!(NameFirst, fakeit::name::first);
varchar_scalar!(NameLast, fakeit::name::last);
varchar_scalar!(NameFull, fakeit::name::full);
varchar_scalar!(NamePrefix, fakeit::name::prefix);
varchar_scalar!(NameSuffix, fakeit::name::suffix);

// Company functions
varchar_scalar!(CompanyCompany, fakeit::company::company);
varchar_scalar!(CompanyBs, fakeit::company::bs);
varchar_scalar!(CompanySuffix, fakeit::company::company_suffix);
varchar_scalar!(CompanyBuzzword, fakeit::company::buzzword);

// Contact functions
varchar_scalar!(ContactEmail, fakeit::contact::email);
varchar_scalar!(ContactPhone, fakeit::contact::phone);
varchar_scalar!(ContactPhoneFormatted, fakeit::contact::phone_formatted);

// Internet functions
varchar_scalar!(InternetIpv4Address, fakeit::internet::ipv4_address);
varchar_scalar!(InternetIpv6Address, fakeit::internet::ipv6_address);
varchar_scalar!(InternetDomainName, fakeit::internet::domain_name);
varchar_scalar!(InternetDomainSuffix, fakeit::internet::domain_suffix);
varchar_scalar!(InternetUsername, fakeit::internet::username);
varchar_scalar!(InternetHttpMethod, fakeit::internet::http_method);
varchar_scalar!(InternetMacAddress, fakeit::internet::mac_address);

// UUID functions
varchar_scalar!(UuidV1, fakeit::unique::uuid_v1);
varchar_scalar!(UuidV4, fakeit::unique::uuid_v4);

// Animal functions
varchar_scalar!(AnimalPetName, fakeit::animal::pet_name);
varchar_scalar!(AnimalAnimal, fakeit::animal::animal);
varchar_scalar!(AnimalFarm, fakeit::animal::farm);
varchar_scalar!(AnimalCat, fakeit::animal::cat);
varchar_scalar!(AnimalDog, fakeit::animal::dog);

// Beer functions
varchar_scalar!(BeerName, fakeit::beer::name);
varchar_scalar!(BeerStyle, fakeit::beer::style);
varchar_scalar!(BeerHop, fakeit::beer::hop);
varchar_scalar!(BeerYeast, fakeit::beer::yeast);
varchar_scalar!(BeerMalt, fakeit::beer::malt);
varchar_scalar!(BeerIbu, fakeit::beer::ibu);
varchar_scalar!(BeerAlcohol, fakeit::beer::alcohol);
varchar_scalar!(BeerBlg, fakeit::beer::blg);

// Color functions
varchar_scalar!(ColorFull, fakeit::color::full);
varchar_scalar!(ColorHex, fakeit::color::hex);
varchar_scalar!(ColorSafe, fakeit::color::safe);
varchar_scalar!(ColorRgb, || {
    let rgb = fakeit::color::rgb();
    format!("[{}, {}, {}]", rgb[0], rgb[1], rgb[2])
});

// Currency functions
varchar_scalar!(CurrencyShort, fakeit::currency::short);
varchar_scalar!(CurrencyLong, fakeit::currency::long);
varchar_scalar!(CurrencyPrice, || format!(
    "{:.2}",
    fakeit::currency::price(0.0, 1000.0)
));

// DateTime functions
varchar_scalar!(DateTimeMonth, fakeit::datetime::month);
varchar_scalar!(DateTimeDay, fakeit::datetime::day);
varchar_scalar!(DateTimeWeekDay, fakeit::datetime::week_day);
varchar_scalar!(DateTimeTimezone, fakeit::datetime::timezone);
varchar_scalar!(DateTimeTimezoneFull, fakeit::datetime::timezone_full);
varchar_scalar!(DateTimeTimezoneAbv, fakeit::datetime::timezone_abv);
varchar_scalar!(DateTimeYear, || fakeit::datetime::year().to_string());
varchar_scalar!(DateTimeHour, || fakeit::datetime::hour().to_string());
varchar_scalar!(DateTimeMinute, || fakeit::datetime::minute().to_string());
varchar_scalar!(DateTimeSecond, || fakeit::datetime::second().to_string());
varchar_scalar!(DateTimeNanosecond, || fakeit::datetime::nanosecond()
    .to_string());
varchar_scalar!(DateTimeTimezoneOffset, || {
    fakeit::datetime::timezone_offset().to_string()
});
varchar_scalar!(DateTimeDate, || format!("{:?}", fakeit::datetime::date()));

// File functions
varchar_scalar!(FileExtension, fakeit::file::extension);
varchar_scalar!(FileMimeType, fakeit::file::mime_type);

// Hacker functions
varchar_scalar!(HackerPhrase, fakeit::hacker::phrase);
varchar_scalar!(HackerAbbreviation, fakeit::hacker::abbreviation);
varchar_scalar!(HackerAdjective, fakeit::hacker::adjective);
varchar_scalar!(HackerNoun, fakeit::hacker::noun);
varchar_scalar!(HackerVerb, fakeit::hacker::verb);
varchar_scalar!(HackerIngverb, fakeit::hacker::ingverb);

// Hipster functions
varchar_scalar!(HipsterWord, fakeit::hipster::word);
varchar_scalar!(HipsterSentence, || fakeit::hipster::sentence(5));
varchar_scalar!(HipsterParagraph, || fakeit::hipster::paragraph(
    3,
    5,
    10,
    String::from(" ")
));

// Image functions
varchar_scalar!(ImageUrl, || fakeit::image::url(640, 480));

// Job functions
varchar_scalar!(JobTitle, fakeit::job::title);
varchar_scalar!(JobDescriptor, fakeit::job::descriptor);
varchar_scalar!(JobLevel, fakeit::job::level);

// Language functions
varchar_scalar!(LanguageRandom, fakeit::language::random);
varchar_scalar!(LanguageAbbreviation, fakeit::language::abbreviation);
varchar_scalar!(LanguageProgramming, fakeit::language::programming);

// Log Level functions
varchar_scalar!(LogLevelGeneral, fakeit::log_level::general);
varchar_scalar!(LogLevelSyslog, fakeit::log_level::syslog);
varchar_scalar!(LogLevelApache, fakeit::log_level::apache);

// Password functions
varchar_scalar!(PasswordGenerate, || fakeit::password::generate(
    true, true, true, 16
));

// Payment functions
varchar_scalar!(PaymentCreditCardType, fakeit::payment::credit_card_type);
varchar_scalar!(PaymentCreditCardNumber, fakeit::payment::credit_card_number);
varchar_scalar!(PaymentCreditCardExp, fakeit::payment::credit_card_exp);
varchar_scalar!(PaymentCreditCardCvv, fakeit::payment::credit_card_cvv);
varchar_scalar!(
    PaymentCreditCardLuhnNumber,
    fakeit::payment::credit_card_luhn_number
);

// Person functions
varchar_scalar!(PersonSsn, fakeit::person::ssn);
varchar_scalar!(PersonGender, fakeit::person::gender);

// User Agent functions
varchar_scalar!(UserAgentChrome, fakeit::user_agent::chrome);
varchar_scalar!(UserAgentFirefox, fakeit::user_agent::firefox);
varchar_scalar!(UserAgentSafari, fakeit::user_agent::safari);
varchar_scalar!(UserAgentOpera, fakeit::user_agent::opera);
varchar_scalar!(
    UserAgentLinuxPlatformToken,
    fakeit::user_agent::linux_platform_token
);
varchar_scalar!(
    UserAgentMacPlatformToken,
    fakeit::user_agent::mac_platform_token
);
varchar_scalar!(
    UserAgentWindowsPlatformToken,
    fakeit::user_agent::windows_platform_token
);
varchar_scalar!(UserAgentRandomPlatform, fakeit::user_agent::random_platform);

// Vehicle functions
varchar_scalar!(VehicleType, fakeit::vehicle::vehicle_type);
varchar_scalar!(VehicleFuel, fakeit::vehicle::fuel);
varchar_scalar!(VehicleTransmissionGear, fakeit::vehicle::transmission_gear);
varchar_scalar!(VehicleCarMaker, fakeit::vehicle::car_maker);
varchar_scalar!(VehicleCarModel, fakeit::vehicle::car_model);

// Words functions
varchar_scalar!(WordsWord, fakeit::words::word);
varchar_scalar!(WordsSentence, || fakeit::words::sentence(10));
varchar_scalar!(WordsParagraph, || fakeit::words::paragraph(
    3,
    5,
    10,
    String::from(" ")
));
varchar_scalar!(WordsQuestion, fakeit::words::question);
varchar_scalar!(WordsQuote, fakeit::words::quote);

// Generator function
varchar_scalar!(GeneratorGenerate, || fakeit::generator::generate(
    "{firstname} {lastname}".to_string()
));

// Boolean function
primitive_scalar!(BoolRand, bool, fakeit::bool_rand::bool);

// Numeric functions
primitive_scalar!(AddressLatitude, f64, || fakeit::address::latitude() as f64);
primitive_scalar!(AddressLongitude, f64, || fakeit::address::longitude()
    as f64);

// Parameterized functions
double_double_scalar!(
    AddressLatitudeInRange,
    |min, max| fakeit::address::latitude_in_range(min as f32, max as f32) as f64
);
double_double_scalar!(
    AddressLongitudeInRange,
    |min, max| fakeit::address::longitude_in_range(min as f32, max as f32) as f64
);

// Status code functions
primitive_scalar!(StatusCodeSimple, i64, || fakeit::status_code::simple()
    as i64);
primitive_scalar!(StatusCodeGeneral, i64, || fakeit::status_code::general()
    as i64);

#[duckdb_entrypoint_c_api_v2(extension_name = "fakeit", api_version = "v2.0.0")]
pub fn extension_entrypoint(extension: &Extension) -> Result<()> {
    // Register all address functions
    AddressStreet::register(
        extension,
        "fakeit_address_street",
        "address",
        "Generates a random street address.",
    )?;
    AddressCity::register(
        extension,
        "fakeit_address_city",
        "address",
        "Generates a random city name.",
    )?;
    AddressState::register(
        extension,
        "fakeit_address_state",
        "address",
        "Generates a random state name.",
    )?;
    AddressZip::register(
        extension,
        "fakeit_address_zip",
        "address",
        "Generates a random postal code.",
    )?;
    AddressCountry::register(
        extension,
        "fakeit_address_country",
        "address",
        "Generates a random country name.",
    )?;
    AddressStreetNumber::register(
        extension,
        "fakeit_address_street_number",
        "address",
        "Generates a random street number as text.",
    )?;
    AddressStreetPrefix::register(
        extension,
        "fakeit_address_street_prefix",
        "address",
        "Generates a random street prefix, such as North or East.",
    )?;
    AddressStreetName::register(
        extension,
        "fakeit_address_street_name",
        "address",
        "Generates a random street name.",
    )?;
    AddressStreetSuffix::register(
        extension,
        "fakeit_address_street_suffix",
        "address",
        "Generates a random street suffix, such as Street or Avenue.",
    )?;
    AddressStateAbr::register(
        extension,
        "fakeit_address_state_abr",
        "address",
        "Generates a random state abbreviation.",
    )?;
    AddressCountryAbr::register(
        extension,
        "fakeit_address_country_abr",
        "address",
        "Generates a random country abbreviation.",
    )?;
    AddressLatitude::register(
        extension,
        "fakeit_address_latitude",
        "address",
        "Generates a random latitude coordinate between -90 and 90 degrees.",
    )?;
    AddressLongitude::register(
        extension,
        "fakeit_address_longitude",
        "address",
        "Generates a random longitude coordinate between -180 and 180 degrees.",
    )?;
    AddressLatitudeInRange::register(extension, "fakeit_address_latitude_in_range", "address", "Generates a latitude using min and max bounds in degrees. Returns NULL if either bound is NULL; invalid geographic bounds fall back to an unrestricted coordinate. The upstream random generator can return values outside the requested interval.")?;
    AddressLongitudeInRange::register(extension, "fakeit_address_longitude_in_range", "address", "Generates a longitude using min and max bounds in degrees. Returns NULL if either bound is NULL; invalid geographic bounds fall back to an unrestricted coordinate. The upstream random generator can return values outside the requested interval.")?;

    NameFirst::register(
        extension,
        "fakeit_name_first",
        "name",
        "Generates a random first name.",
    )?;
    NameLast::register(
        extension,
        "fakeit_name_last",
        "name",
        "Generates a random last name.",
    )?;
    NameFull::register(
        extension,
        "fakeit_name_full",
        "name",
        "Generates a random full name with a first and last name.",
    )?;
    NamePrefix::register(
        extension,
        "fakeit_name_prefix",
        "name",
        "Generates a random name prefix, such as Mr. or Dr.",
    )?;
    NameSuffix::register(
        extension,
        "fakeit_name_suffix",
        "name",
        "Generates a random name suffix, such as Jr. or III.",
    )?;

    CompanyCompany::register(
        extension,
        "fakeit_company_company",
        "company",
        "Generates a random company name.",
    )?;
    CompanyBs::register(
        extension,
        "fakeit_company_bs",
        "company",
        "Generates a random business statement.",
    )?;
    CompanySuffix::register(
        extension,
        "fakeit_company_company_suffix",
        "company",
        "Generates a random company suffix, such as Inc or LLC.",
    )?;
    CompanyBuzzword::register(
        extension,
        "fakeit_company_buzzword",
        "company",
        "Generates a random business buzzword.",
    )?;

    ContactEmail::register(
        extension,
        "fakeit_contact_email",
        "contact",
        "Generates a random email address for test data.",
    )?;
    ContactPhone::register(
        extension,
        "fakeit_contact_phone",
        "contact",
        "Generates a random unformatted phone number.",
    )?;
    ContactPhoneFormatted::register(
        extension,
        "fakeit_contact_phone_formatted",
        "contact",
        "Generates a random formatted phone number.",
    )?;

    InternetIpv4Address::register(
        extension,
        "fakeit_internet_ipv4_address",
        "internet",
        "Generates a random IPv4 address.",
    )?;
    InternetIpv6Address::register(
        extension,
        "fakeit_internet_ipv6_address",
        "internet",
        "Generates a random IPv6 address.",
    )?;
    InternetDomainName::register(
        extension,
        "fakeit_internet_domain_name",
        "internet",
        "Generates a random domain name.",
    )?;
    InternetDomainSuffix::register(
        extension,
        "fakeit_internet_domain_suffix",
        "internet",
        "Generates a random domain suffix, such as com or org.",
    )?;
    InternetUsername::register(
        extension,
        "fakeit_internet_username",
        "internet",
        "Generates a random username.",
    )?;
    InternetHttpMethod::register(
        extension,
        "fakeit_internet_http_method",
        "internet",
        "Selects a random HTTP method, such as GET or POST.",
    )?;
    InternetMacAddress::register(
        extension,
        "fakeit_internet_mac_address",
        "internet",
        "Generates a random MAC address.",
    )?;

    UuidV1::register(
        extension,
        "fakeit_uuid_v1",
        "uuid",
        "Generates a version 1 UUID using the current time and a fixed node identifier.",
    )?;
    UuidV4::register(
        extension,
        "fakeit_uuid_v4",
        "uuid",
        "Generates a random version 4 UUID.",
    )?;

    AnimalPetName::register(
        extension,
        "fakeit_animal_pet_name",
        "animal",
        "Generates a random pet name.",
    )?;
    AnimalAnimal::register(
        extension,
        "fakeit_animal_animal",
        "animal",
        "Selects a random animal name.",
    )?;
    AnimalFarm::register(
        extension,
        "fakeit_animal_farm",
        "animal",
        "Selects a random farm animal.",
    )?;
    AnimalCat::register(
        extension,
        "fakeit_animal_cat",
        "animal",
        "Selects a random cat breed.",
    )?;
    AnimalDog::register(
        extension,
        "fakeit_animal_dog",
        "animal",
        "Selects a random dog breed.",
    )?;

    BeerName::register(
        extension,
        "fakeit_beer_name",
        "beer",
        "Selects a random beer name.",
    )?;
    BeerStyle::register(
        extension,
        "fakeit_beer_style",
        "beer",
        "Selects a random beer style.",
    )?;
    BeerHop::register(
        extension,
        "fakeit_beer_hop",
        "beer",
        "Selects a random hop variety.",
    )?;
    BeerYeast::register(
        extension,
        "fakeit_beer_yeast",
        "beer",
        "Selects a random yeast type.",
    )?;
    BeerMalt::register(
        extension,
        "fakeit_beer_malt",
        "beer",
        "Selects a random malt variety.",
    )?;
    BeerIbu::register(
        extension,
        "fakeit_beer_ibu",
        "beer",
        "Generates a random beer bitterness value as text with an IBU suffix.",
    )?;
    BeerAlcohol::register(
        extension,
        "fakeit_beer_alcohol",
        "beer",
        "Generates a random alcohol percentage as text with a percent sign.",
    )?;
    BeerBlg::register(
        extension,
        "fakeit_beer_blg",
        "beer",
        "Generates a random beer gravity value as text in degrees Balling.",
    )?;

    ColorFull::register(
        extension,
        "fakeit_color_full",
        "color",
        "Selects a random color name.",
    )?;
    ColorHex::register(
        extension,
        "fakeit_color_hex",
        "color",
        "Generates a random hexadecimal color code.",
    )?;
    ColorSafe::register(
        extension,
        "fakeit_color_safe",
        "color",
        "Selects a random web-safe color name.",
    )?;
    ColorRgb::register(
        extension,
        "fakeit_color_rgb",
        "color",
        "Generates a random RGB color as text in the form [R, G, B].",
    )?;

    CurrencyShort::register(
        extension,
        "fakeit_currency_short",
        "currency",
        "Selects a random currency code, such as USD.",
    )?;
    CurrencyLong::register(
        extension,
        "fakeit_currency_long",
        "currency",
        "Selects a random currency name, such as US Dollar.",
    )?;
    CurrencyPrice::register(extension, "fakeit_currency_price", "currency", "Generates a price using fixed bounds of 0 and 1000, formatted as text with two decimal places.")?;

    DateTimeMonth::register(
        extension,
        "fakeit_datetime_month",
        "datetime",
        "Generates a random month number as text.",
    )?;
    DateTimeDay::register(
        extension,
        "fakeit_datetime_day",
        "datetime",
        "Generates a random day-of-month number as text.",
    )?;
    DateTimeWeekDay::register(
        extension,
        "fakeit_datetime_week_day",
        "datetime",
        "Generates a random numeric weekday value as text.",
    )?;
    DateTimeTimezone::register(
        extension,
        "fakeit_datetime_timezone",
        "datetime",
        "Selects a random timezone name.",
    )?;
    DateTimeTimezoneFull::register(
        extension,
        "fakeit_datetime_timezone_full",
        "datetime",
        "Selects a random full timezone description.",
    )?;
    DateTimeTimezoneAbv::register(
        extension,
        "fakeit_datetime_timezone_abv",
        "datetime",
        "Selects a random timezone abbreviation.",
    )?;
    DateTimeYear::register(
        extension,
        "fakeit_datetime_year",
        "datetime",
        "Generates a random year as text.",
    )?;
    DateTimeHour::register(
        extension,
        "fakeit_datetime_hour",
        "datetime",
        "Generates a random hour as text.",
    )?;
    DateTimeMinute::register(
        extension,
        "fakeit_datetime_minute",
        "datetime",
        "Generates a random minute as text.",
    )?;
    DateTimeSecond::register(
        extension,
        "fakeit_datetime_second",
        "datetime",
        "Generates a random second as text.",
    )?;
    DateTimeNanosecond::register(
        extension,
        "fakeit_datetime_nanosecond",
        "datetime",
        "Generates a random nanosecond value as text.",
    )?;
    DateTimeTimezoneOffset::register(
        extension,
        "fakeit_datetime_timezone_offset",
        "datetime",
        "Selects a random timezone offset as text.",
    )?;
    DateTimeDate::register(
        extension,
        "fakeit_datetime_date",
        "datetime",
        "Returns a generated date as text with secs and nsecs fields.",
    )?;

    FileExtension::register(
        extension,
        "fakeit_file_extension",
        "file",
        "Selects a random file extension.",
    )?;
    FileMimeType::register(
        extension,
        "fakeit_file_mime_type",
        "file",
        "Selects a random MIME type.",
    )?;

    HackerPhrase::register(
        extension,
        "fakeit_hacker_phrase",
        "hacker",
        "Generates a random hacker-style phrase.",
    )?;
    HackerAbbreviation::register(
        extension,
        "fakeit_hacker_abbreviation",
        "hacker",
        "Selects a random technical abbreviation.",
    )?;
    HackerAdjective::register(
        extension,
        "fakeit_hacker_adjective",
        "hacker",
        "Selects a random technical adjective.",
    )?;
    HackerNoun::register(
        extension,
        "fakeit_hacker_noun",
        "hacker",
        "Selects a random technical noun.",
    )?;
    HackerVerb::register(
        extension,
        "fakeit_hacker_verb",
        "hacker",
        "Selects a random technical verb.",
    )?;
    HackerIngverb::register(
        extension,
        "fakeit_hacker_ingverb",
        "hacker",
        "Selects a random technical verb ending in -ing.",
    )?;

    HipsterWord::register(
        extension,
        "fakeit_hipster_word",
        "hipster",
        "Selects a random hipster word.",
    )?;
    HipsterSentence::register(
        extension,
        "fakeit_hipster_sentence",
        "hipster",
        "Generates a five-word sentence using the upstream hipster sentence generator.",
    )?;
    HipsterParagraph::register(
        extension,
        "fakeit_hipster_paragraph",
        "hipster",
        "Generates three paragraphs of five ten-word sentences each, joined with spaces.",
    )?;

    ImageUrl::register(
        extension,
        "fakeit_image_url",
        "image",
        "Returns a picsum.photos placeholder image URL with fixed dimensions of 640 by 480 pixels.",
    )?;

    JobTitle::register(
        extension,
        "fakeit_job_title",
        "job",
        "Selects a random job title.",
    )?;
    JobDescriptor::register(
        extension,
        "fakeit_job_descriptor",
        "job",
        "Selects a random job descriptor.",
    )?;
    JobLevel::register(
        extension,
        "fakeit_job_level",
        "job",
        "Selects a random job level.",
    )?;

    LanguageRandom::register(
        extension,
        "fakeit_language_random",
        "language",
        "Selects a random language name.",
    )?;
    LanguageAbbreviation::register(
        extension,
        "fakeit_language_abbreviation",
        "language",
        "Selects a random language abbreviation.",
    )?;
    LanguageProgramming::register(
        extension,
        "fakeit_language_programming",
        "language",
        "Selects a random programming language.",
    )?;

    LogLevelGeneral::register(
        extension,
        "fakeit_log_level_general",
        "log_level",
        "Selects a random general log level.",
    )?;
    LogLevelSyslog::register(
        extension,
        "fakeit_log_level_syslog",
        "log_level",
        "Selects a random syslog severity level.",
    )?;
    LogLevelApache::register(
        extension,
        "fakeit_log_level_apache",
        "log_level",
        "Selects a random Apache log level.",
    )?;

    PasswordGenerate::register(extension, "fakeit_password_generate", "password", "Generates a test password from letters, digits, and special characters using fixed generator settings.")?;

    PaymentCreditCardType::register(
        extension,
        "fakeit_payment_credit_card_type",
        "payment",
        "Selects a random credit card type for test data.",
    )?;
    PaymentCreditCardNumber::register(
        extension,
        "fakeit_payment_credit_card_number",
        "payment",
        "Generates a random credit card number for test data.",
    )?;
    PaymentCreditCardExp::register(
        extension,
        "fakeit_payment_credit_card_exp",
        "payment",
        "Generates a test credit card expiration date in MM/YY format.",
    )?;
    PaymentCreditCardCvv::register(
        extension,
        "fakeit_payment_credit_card_cvv",
        "payment",
        "Generates a three-digit credit card verification code for test data.",
    )?;
    PaymentCreditCardLuhnNumber::register(
        extension,
        "fakeit_payment_credit_card_luhn_number",
        "payment",
        "Generates a test card number using the upstream Luhn-based generator.",
    )?;

    PersonSsn::register(
        extension,
        "fakeit_person_ssn",
        "person",
        "Generates a random Social Security number for test data.",
    )?;
    PersonGender::register(
        extension,
        "fakeit_person_gender",
        "person",
        "Selects a random gender value from the upstream dataset.",
    )?;

    UserAgentChrome::register(
        extension,
        "fakeit_user_agent_chrome",
        "user_agent",
        "Generates a random Chrome user-agent string.",
    )?;
    UserAgentFirefox::register(
        extension,
        "fakeit_user_agent_firefox",
        "user_agent",
        "Generates a random Firefox user-agent string.",
    )?;
    UserAgentSafari::register(
        extension,
        "fakeit_user_agent_safari",
        "user_agent",
        "Generates a random Safari user-agent string.",
    )?;
    UserAgentOpera::register(
        extension,
        "fakeit_user_agent_opera",
        "user_agent",
        "Generates a random Opera user-agent string.",
    )?;
    UserAgentLinuxPlatformToken::register(
        extension,
        "fakeit_user_agent_linux_platform_token",
        "user_agent",
        "Generates a random Linux platform token for a user-agent string.",
    )?;
    UserAgentMacPlatformToken::register(
        extension,
        "fakeit_user_agent_mac_platform_token",
        "user_agent",
        "Generates a random macOS platform token for a user-agent string.",
    )?;
    UserAgentWindowsPlatformToken::register(
        extension,
        "fakeit_user_agent_windows_platform_token",
        "user_agent",
        "Generates a random Windows platform token for a user-agent string.",
    )?;
    UserAgentRandomPlatform::register(
        extension,
        "fakeit_user_agent_random_platform",
        "user_agent",
        "Selects a random platform token for a user-agent string.",
    )?;

    VehicleType::register(
        extension,
        "fakeit_vehicle_vehicle_type",
        "vehicle",
        "Selects a random vehicle type.",
    )?;
    VehicleFuel::register(
        extension,
        "fakeit_vehicle_fuel",
        "vehicle",
        "Selects a random vehicle fuel type.",
    )?;
    VehicleTransmissionGear::register(
        extension,
        "fakeit_vehicle_transmission_gear",
        "vehicle",
        "Selects a random vehicle transmission type.",
    )?;
    VehicleCarMaker::register(
        extension,
        "fakeit_vehicle_car_maker",
        "vehicle",
        "Selects a random car manufacturer.",
    )?;
    VehicleCarModel::register(
        extension,
        "fakeit_vehicle_car_model",
        "vehicle",
        "Selects a random car model.",
    )?;

    WordsWord::register(
        extension,
        "fakeit_words_word",
        "words",
        "Selects a random word.",
    )?;
    WordsSentence::register(
        extension,
        "fakeit_words_sentence",
        "words",
        "Generates a random ten-word sentence.",
    )?;
    WordsParagraph::register(
        extension,
        "fakeit_words_paragraph",
        "words",
        "Generates three paragraphs of five ten-word sentences each, joined with spaces.",
    )?;
    WordsQuestion::register(
        extension,
        "fakeit_words_question",
        "words",
        "Generates a random question ending with a question mark.",
    )?;
    WordsQuote::register(
        extension,
        "fakeit_words_quote",
        "words",
        "Generates a random quotation with a generated author name.",
    )?;

    GeneratorGenerate::register(extension, "fakeit_generator_generate", "generator", "Generates a first and last name using the fixed template {firstname} {lastname}; accepts no arguments.")?;

    BoolRand::register(
        extension,
        "fakeit_bool",
        "boolean",
        "Generates a random boolean value.",
    )?;

    StatusCodeSimple::register(
        extension,
        "fakeit_status_code_simple",
        "status_code",
        "Selects a random common HTTP status code as a BIGINT.",
    )?;
    StatusCodeGeneral::register(
        extension,
        "fakeit_status_code_general",
        "status_code",
        "Selects a random HTTP status code from the general set as a BIGINT.",
    )?;

    Ok(())
}
