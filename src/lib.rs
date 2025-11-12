extern crate duckdb;
extern crate duckdb_loadable_macros;
extern crate libduckdb_sys;

use duckdb::{Connection, Result};
use duckdb::core::{DataChunkHandle, Inserter, LogicalTypeHandle, LogicalTypeId};
use duckdb::vscalar::{ScalarFunctionSignature, VScalar};
use duckdb::vtab::arrow::WritableVector;
use duckdb_loadable_macros::duckdb_entrypoint_c_api;
use libduckdb_sys as ffi;
use std::error::Error;

// Macro to create a zero-argument VARCHAR scalar function struct
macro_rules! varchar_scalar {
    ($name:ident, $func:expr) => {
        struct $name;

        impl VScalar for $name {
            type State = ();

            unsafe fn invoke(
                _state: &Self::State,
                input: &mut DataChunkHandle,
                output: &mut dyn WritableVector,
            ) -> Result<(), Box<dyn Error>> {
                let len = input.len();
                let output_vec = output.flat_vector();

                for i in 0..len {
                    let result: String = $func();
                    output_vec.insert(i, result.as_str());
                }
                Ok(())
            }

            fn signatures() -> Vec<ScalarFunctionSignature> {
                vec![ScalarFunctionSignature::exact(
                    vec![],
                    LogicalTypeHandle::from(LogicalTypeId::Varchar),
                )]
            }
        }
    };
}

// Macro to create a zero-argument BIGINT scalar function struct
macro_rules! bigint_scalar {
    ($name:ident, $func:expr) => {
        struct $name;

        impl VScalar for $name {
            type State = ();

            unsafe fn invoke(
                _state: &Self::State,
                input: &mut DataChunkHandle,
                output: &mut dyn WritableVector,
            ) -> Result<(), Box<dyn Error>> {
                let len = input.len();
                let mut output_vec = output.flat_vector();
                let output_data = output_vec.as_mut_slice::<i64>();

                for i in 0..len {
                    output_data[i] = $func() as i64;
                }
                Ok(())
            }

            fn signatures() -> Vec<ScalarFunctionSignature> {
                vec![ScalarFunctionSignature::exact(
                    vec![],
                    LogicalTypeHandle::from(LogicalTypeId::Bigint),
                )]
            }
        }
    };
}

// Macro to create a zero-argument DOUBLE scalar function struct
macro_rules! double_scalar {
    ($name:ident, $func:expr) => {
        struct $name;

        impl VScalar for $name {
            type State = ();

            unsafe fn invoke(
                _state: &Self::State,
                input: &mut DataChunkHandle,
                output: &mut dyn WritableVector,
            ) -> Result<(), Box<dyn Error>> {
                let len = input.len();
                let mut output_vec = output.flat_vector();
                let output_data = output_vec.as_mut_slice::<f64>();

                for i in 0..len {
                    output_data[i] = $func();
                }
                Ok(())
            }

            fn signatures() -> Vec<ScalarFunctionSignature> {
                vec![ScalarFunctionSignature::exact(
                    vec![],
                    LogicalTypeHandle::from(LogicalTypeId::Double),
                )]
            }
        }
    };
}

// Macro to create a zero-argument BOOLEAN scalar function struct
macro_rules! boolean_scalar {
    ($name:ident, $func:expr) => {
        struct $name;

        impl VScalar for $name {
            type State = ();

            unsafe fn invoke(
                _state: &Self::State,
                input: &mut DataChunkHandle,
                output: &mut dyn WritableVector,
            ) -> Result<(), Box<dyn Error>> {
                let len = input.len();
                let mut output_vec = output.flat_vector();
                let output_data = output_vec.as_mut_slice::<bool>();

                for i in 0..len {
                    output_data[i] = $func();
                }
                Ok(())
            }

            fn signatures() -> Vec<ScalarFunctionSignature> {
                vec![ScalarFunctionSignature::exact(
                    vec![],
                    LogicalTypeHandle::from(LogicalTypeId::Boolean),
                )]
            }
        }
    };
}

// Macro to create a two-DOUBLE-argument DOUBLE scalar function struct
macro_rules! double_double_scalar {
    ($name:ident, $func:expr) => {
        struct $name;

        impl VScalar for $name {
            type State = ();

            unsafe fn invoke(
                _state: &Self::State,
                input: &mut DataChunkHandle,
                output: &mut dyn WritableVector,
            ) -> Result<(), Box<dyn Error>> {
                let len = input.len();
                let input1 = input.flat_vector(0);
                let input2 = input.flat_vector(1);
                let input_data1 = input1.as_slice::<f64>();
                let input_data2 = input2.as_slice::<f64>();

                let mut output_vec = output.flat_vector();
                let output_data = output_vec.as_mut_slice::<f64>();

                for i in 0..len {
                    output_data[i] = $func(input_data1[i], input_data2[i]);
                }
                Ok(())
            }

            fn signatures() -> Vec<ScalarFunctionSignature> {
                vec![ScalarFunctionSignature::exact(
                    vec![
                        LogicalTypeHandle::from(LogicalTypeId::Double),
                        LogicalTypeHandle::from(LogicalTypeId::Double),
                    ],
                    LogicalTypeHandle::from(LogicalTypeId::Double),
                )]
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
varchar_scalar!(CurrencyPrice, || format!("{:.2}", fakeit::currency::price(0.0, 1000.0)));

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
varchar_scalar!(DateTimeNanosecond, || fakeit::datetime::nanosecond().to_string());
varchar_scalar!(DateTimeTimezoneOffset, || fakeit::datetime::timezone_offset().to_string());
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
varchar_scalar!(HipsterParagraph, || fakeit::hipster::paragraph(3, 5, 10, String::from(" ")));

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
varchar_scalar!(PasswordGenerate, || fakeit::password::generate(true, true, true, 16));

// Payment functions
varchar_scalar!(PaymentCreditCardType, fakeit::payment::credit_card_type);
varchar_scalar!(PaymentCreditCardNumber, fakeit::payment::credit_card_number);
varchar_scalar!(PaymentCreditCardExp, fakeit::payment::credit_card_exp);
varchar_scalar!(PaymentCreditCardCvv, fakeit::payment::credit_card_cvv);
varchar_scalar!(PaymentCreditCardLuhnNumber, fakeit::payment::credit_card_luhn_number);

// Person functions
varchar_scalar!(PersonSsn, fakeit::person::ssn);
varchar_scalar!(PersonGender, fakeit::person::gender);

// User Agent functions
varchar_scalar!(UserAgentChrome, fakeit::user_agent::chrome);
varchar_scalar!(UserAgentFirefox, fakeit::user_agent::firefox);
varchar_scalar!(UserAgentSafari, fakeit::user_agent::safari);
varchar_scalar!(UserAgentOpera, fakeit::user_agent::opera);
varchar_scalar!(UserAgentLinuxPlatformToken, fakeit::user_agent::linux_platform_token);
varchar_scalar!(UserAgentMacPlatformToken, fakeit::user_agent::mac_platform_token);
varchar_scalar!(UserAgentWindowsPlatformToken, fakeit::user_agent::windows_platform_token);
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
varchar_scalar!(WordsParagraph, || fakeit::words::paragraph(3, 5, 10, String::from(" ")));
varchar_scalar!(WordsQuestion, fakeit::words::question);
varchar_scalar!(WordsQuote, fakeit::words::quote);

// Generator function
varchar_scalar!(GeneratorGenerate, || fakeit::generator::generate("{firstname} {lastname}".to_string()));

// Boolean function
boolean_scalar!(BoolRand, fakeit::bool_rand::bool);

// Numeric functions
double_scalar!(AddressLatitude, || fakeit::address::latitude() as f64);
double_scalar!(AddressLongitude, || fakeit::address::longitude() as f64);

// Parameterized functions
double_double_scalar!(AddressLatitudeInRange, |min, max| fakeit::address::latitude_in_range(min as f32, max as f32) as f64);
double_double_scalar!(AddressLongitudeInRange, |min, max| fakeit::address::longitude_in_range(min as f32, max as f32) as f64);

// Status code functions
bigint_scalar!(StatusCodeSimple, || fakeit::status_code::simple() as i64);
bigint_scalar!(StatusCodeGeneral, || fakeit::status_code::general() as i64);

// Note: Functions currently produce duplicate values when used with generate_series()
// This is because DuckDB optimizes zero-argument functions as constants.
//
// To fix this, we need to mark functions as VOLATILE using duckdb_scalar_function_set_volatile()
// However, duckdb-rs doesn't currently expose this in its public API.
//
// TODO: Submit PR to duckdb-rs to add set_volatile() method to ScalarFunction
// See: https://github.com/duckdb/duckdb-rs for contribution guidelines

#[duckdb_entrypoint_c_api()]
pub unsafe fn extension_entrypoint(con: Connection) -> Result<(), Box<dyn Error>> {
    // Register all address functions
    con.register_scalar_function::<AddressStreet>("fakeit_address_street")?;
    con.register_scalar_function::<AddressCity>("fakeit_address_city")?;
    con.register_scalar_function::<AddressState>("fakeit_address_state")?;
    con.register_scalar_function::<AddressZip>("fakeit_address_zip")?;
    con.register_scalar_function::<AddressCountry>("fakeit_address_country")?;
    con.register_scalar_function::<AddressStreetNumber>("fakeit_address_street_number")?;
    con.register_scalar_function::<AddressStreetPrefix>("fakeit_address_street_prefix")?;
    con.register_scalar_function::<AddressStreetName>("fakeit_address_street_name")?;
    con.register_scalar_function::<AddressStreetSuffix>("fakeit_address_street_suffix")?;
    con.register_scalar_function::<AddressStateAbr>("fakeit_address_state_abr")?;
    con.register_scalar_function::<AddressCountryAbr>("fakeit_address_country_abr")?;
    con.register_scalar_function::<AddressLatitude>("fakeit_address_latitude")?;
    con.register_scalar_function::<AddressLongitude>("fakeit_address_longitude")?;
    con.register_scalar_function::<AddressLatitudeInRange>("fakeit_address_latitude_in_range")?;
    con.register_scalar_function::<AddressLongitudeInRange>("fakeit_address_longitude_in_range")?;

    con.register_scalar_function::<NameFirst>("fakeit_name_first")?;
    con.register_scalar_function::<NameLast>("fakeit_name_last")?;
    con.register_scalar_function::<NameFull>("fakeit_name_full")?;
    con.register_scalar_function::<NamePrefix>("fakeit_name_prefix")?;
    con.register_scalar_function::<NameSuffix>("fakeit_name_suffix")?;

    con.register_scalar_function::<CompanyCompany>("fakeit_company_company")?;
    con.register_scalar_function::<CompanyBs>("fakeit_company_bs")?;
    con.register_scalar_function::<CompanySuffix>("fakeit_company_company_suffix")?;
    con.register_scalar_function::<CompanyBuzzword>("fakeit_company_buzzword")?;

    con.register_scalar_function::<ContactEmail>("fakeit_contact_email")?;
    con.register_scalar_function::<ContactPhone>("fakeit_contact_phone")?;
    con.register_scalar_function::<ContactPhoneFormatted>("fakeit_contact_phone_formatted")?;

    con.register_scalar_function::<InternetIpv4Address>("fakeit_internet_ipv4_address")?;
    con.register_scalar_function::<InternetIpv6Address>("fakeit_internet_ipv6_address")?;
    con.register_scalar_function::<InternetDomainName>("fakeit_internet_domain_name")?;
    con.register_scalar_function::<InternetDomainSuffix>("fakeit_internet_domain_suffix")?;
    con.register_scalar_function::<InternetUsername>("fakeit_internet_username")?;
    con.register_scalar_function::<InternetHttpMethod>("fakeit_internet_http_method")?;
    con.register_scalar_function::<InternetMacAddress>("fakeit_internet_mac_address")?;

    con.register_scalar_function::<UuidV1>("fakeit_uuid_v1")?;
    con.register_scalar_function::<UuidV4>("fakeit_uuid_v4")?;

    con.register_scalar_function::<AnimalPetName>("fakeit_animal_pet_name")?;
    con.register_scalar_function::<AnimalAnimal>("fakeit_animal_animal")?;
    con.register_scalar_function::<AnimalFarm>("fakeit_animal_farm")?;
    con.register_scalar_function::<AnimalCat>("fakeit_animal_cat")?;
    con.register_scalar_function::<AnimalDog>("fakeit_animal_dog")?;

    con.register_scalar_function::<BeerName>("fakeit_beer_name")?;
    con.register_scalar_function::<BeerStyle>("fakeit_beer_style")?;
    con.register_scalar_function::<BeerHop>("fakeit_beer_hop")?;
    con.register_scalar_function::<BeerYeast>("fakeit_beer_yeast")?;
    con.register_scalar_function::<BeerMalt>("fakeit_beer_malt")?;
    con.register_scalar_function::<BeerIbu>("fakeit_beer_ibu")?;
    con.register_scalar_function::<BeerAlcohol>("fakeit_beer_alcohol")?;
    con.register_scalar_function::<BeerBlg>("fakeit_beer_blg")?;

    con.register_scalar_function::<ColorFull>("fakeit_color_full")?;
    con.register_scalar_function::<ColorHex>("fakeit_color_hex")?;
    con.register_scalar_function::<ColorSafe>("fakeit_color_safe")?;
    con.register_scalar_function::<ColorRgb>("fakeit_color_rgb")?;

    con.register_scalar_function::<CurrencyShort>("fakeit_currency_short")?;
    con.register_scalar_function::<CurrencyLong>("fakeit_currency_long")?;
    con.register_scalar_function::<CurrencyPrice>("fakeit_currency_price")?;

    con.register_scalar_function::<DateTimeMonth>("fakeit_datetime_month")?;
    con.register_scalar_function::<DateTimeDay>("fakeit_datetime_day")?;
    con.register_scalar_function::<DateTimeWeekDay>("fakeit_datetime_week_day")?;
    con.register_scalar_function::<DateTimeTimezone>("fakeit_datetime_timezone")?;
    con.register_scalar_function::<DateTimeTimezoneFull>("fakeit_datetime_timezone_full")?;
    con.register_scalar_function::<DateTimeTimezoneAbv>("fakeit_datetime_timezone_abv")?;
    con.register_scalar_function::<DateTimeYear>("fakeit_datetime_year")?;
    con.register_scalar_function::<DateTimeHour>("fakeit_datetime_hour")?;
    con.register_scalar_function::<DateTimeMinute>("fakeit_datetime_minute")?;
    con.register_scalar_function::<DateTimeSecond>("fakeit_datetime_second")?;
    con.register_scalar_function::<DateTimeNanosecond>("fakeit_datetime_nanosecond")?;
    con.register_scalar_function::<DateTimeTimezoneOffset>("fakeit_datetime_timezone_offset")?;
    con.register_scalar_function::<DateTimeDate>("fakeit_datetime_date")?;

    con.register_scalar_function::<FileExtension>("fakeit_file_extension")?;
    con.register_scalar_function::<FileMimeType>("fakeit_file_mime_type")?;

    con.register_scalar_function::<HackerPhrase>("fakeit_hacker_phrase")?;
    con.register_scalar_function::<HackerAbbreviation>("fakeit_hacker_abbreviation")?;
    con.register_scalar_function::<HackerAdjective>("fakeit_hacker_adjective")?;
    con.register_scalar_function::<HackerNoun>("fakeit_hacker_noun")?;
    con.register_scalar_function::<HackerVerb>("fakeit_hacker_verb")?;
    con.register_scalar_function::<HackerIngverb>("fakeit_hacker_ingverb")?;

    con.register_scalar_function::<HipsterWord>("fakeit_hipster_word")?;
    con.register_scalar_function::<HipsterSentence>("fakeit_hipster_sentence")?;
    con.register_scalar_function::<HipsterParagraph>("fakeit_hipster_paragraph")?;

    con.register_scalar_function::<ImageUrl>("fakeit_image_url")?;

    con.register_scalar_function::<JobTitle>("fakeit_job_title")?;
    con.register_scalar_function::<JobDescriptor>("fakeit_job_descriptor")?;
    con.register_scalar_function::<JobLevel>("fakeit_job_level")?;

    con.register_scalar_function::<LanguageRandom>("fakeit_language_random")?;
    con.register_scalar_function::<LanguageAbbreviation>("fakeit_language_abbreviation")?;
    con.register_scalar_function::<LanguageProgramming>("fakeit_language_programming")?;

    con.register_scalar_function::<LogLevelGeneral>("fakeit_log_level_general")?;
    con.register_scalar_function::<LogLevelSyslog>("fakeit_log_level_syslog")?;
    con.register_scalar_function::<LogLevelApache>("fakeit_log_level_apache")?;

    con.register_scalar_function::<PasswordGenerate>("fakeit_password_generate")?;

    con.register_scalar_function::<PaymentCreditCardType>("fakeit_payment_credit_card_type")?;
    con.register_scalar_function::<PaymentCreditCardNumber>("fakeit_payment_credit_card_number")?;
    con.register_scalar_function::<PaymentCreditCardExp>("fakeit_payment_credit_card_exp")?;
    con.register_scalar_function::<PaymentCreditCardCvv>("fakeit_payment_credit_card_cvv")?;
    con.register_scalar_function::<PaymentCreditCardLuhnNumber>("fakeit_payment_credit_card_luhn_number")?;

    con.register_scalar_function::<PersonSsn>("fakeit_person_ssn")?;
    con.register_scalar_function::<PersonGender>("fakeit_person_gender")?;

    con.register_scalar_function::<UserAgentChrome>("fakeit_user_agent_chrome")?;
    con.register_scalar_function::<UserAgentFirefox>("fakeit_user_agent_firefox")?;
    con.register_scalar_function::<UserAgentSafari>("fakeit_user_agent_safari")?;
    con.register_scalar_function::<UserAgentOpera>("fakeit_user_agent_opera")?;
    con.register_scalar_function::<UserAgentLinuxPlatformToken>("fakeit_user_agent_linux_platform_token")?;
    con.register_scalar_function::<UserAgentMacPlatformToken>("fakeit_user_agent_mac_platform_token")?;
    con.register_scalar_function::<UserAgentWindowsPlatformToken>("fakeit_user_agent_windows_platform_token")?;
    con.register_scalar_function::<UserAgentRandomPlatform>("fakeit_user_agent_random_platform")?;

    con.register_scalar_function::<VehicleType>("fakeit_vehicle_vehicle_type")?;
    con.register_scalar_function::<VehicleFuel>("fakeit_vehicle_fuel")?;
    con.register_scalar_function::<VehicleTransmissionGear>("fakeit_vehicle_transmission_gear")?;
    con.register_scalar_function::<VehicleCarMaker>("fakeit_vehicle_car_maker")?;
    con.register_scalar_function::<VehicleCarModel>("fakeit_vehicle_car_model")?;

    con.register_scalar_function::<WordsWord>("fakeit_words_word")?;
    con.register_scalar_function::<WordsSentence>("fakeit_words_sentence")?;
    con.register_scalar_function::<WordsParagraph>("fakeit_words_paragraph")?;
    con.register_scalar_function::<WordsQuestion>("fakeit_words_question")?;
    con.register_scalar_function::<WordsQuote>("fakeit_words_quote")?;

    con.register_scalar_function::<GeneratorGenerate>("fakeit_generator_generate")?;

    con.register_scalar_function::<BoolRand>("fakeit_bool")?;

    con.register_scalar_function::<StatusCodeSimple>("fakeit_status_code_simple")?;
    con.register_scalar_function::<StatusCodeGeneral>("fakeit_status_code_general")?;

    Ok(())
}
