extern crate duckdb;
extern crate duckdb_loadable_macros;
extern crate libduckdb_sys;

use duckdb::{Connection, Result};
use duckdb_loadable_macros::duckdb_entrypoint_c_api;
use libduckdb_sys as ffi;
use std::{
    error::Error,
    ffi::CString,
};

const EXTENSION_NAME: &str = env!("CARGO_PKG_NAME");

// Helper function to register a scalar function that takes no arguments and returns VARCHAR
unsafe fn register_varchar_fn(
    con: ffi::duckdb_connection,
    name: &str,
    func: fn() -> String,
) -> Result<(), Box<dyn Error>>
{
    unsafe extern "C" fn wrapper(
        _context: ffi::duckdb_function_info,
        input: ffi::duckdb_data_chunk,
        output: ffi::duckdb_vector,
    ) {
        let size = ffi::duckdb_data_chunk_get_size(input);
        let func_ptr = ffi::duckdb_scalar_function_get_extra_info(_context) as *const fn() -> String;
        let func = *func_ptr;
        let result = func();
        let c_str = CString::new(result).unwrap();

        for i in 0..size {
            ffi::duckdb_vector_assign_string_element_len(
                output,
                i,
                c_str.as_ptr(),
                c_str.as_bytes().len() as u64,
            );
        }
    }

    let name_c = CString::new(name)?;
    let scalar_function = ffi::duckdb_create_scalar_function();

    ffi::duckdb_scalar_function_set_name(scalar_function, name_c.as_ptr());

    let return_type = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR);
    ffi::duckdb_scalar_function_set_return_type(scalar_function, return_type);
    ffi::duckdb_destroy_logical_type(&mut (return_type as *mut _));

    // Store function pointer in extra info
    let func_box = Box::new(func);
    let func_ptr = Box::into_raw(func_box) as *mut std::ffi::c_void;
    ffi::duckdb_scalar_function_set_extra_info(
        scalar_function,
        func_ptr,
        None,
    );

    ffi::duckdb_scalar_function_set_function(scalar_function, Some(wrapper));

    let state = ffi::duckdb_register_scalar_function(con, scalar_function);
    ffi::duckdb_destroy_scalar_function(&mut (scalar_function as *mut _));

    if state != ffi::DuckDBSuccess {
        return Err(format!("Failed to register function: {}", name).into());
    }

    Ok(())
}

#[duckdb_entrypoint_c_api()]
pub unsafe fn extension_entrypoint(con: Connection) -> Result<(), Box<dyn Error>> {
    // Get the raw connection handle
    let raw_con = std::mem::transmute::<&Connection, &ffi::duckdb_connection>(&con);
    let con_handle = *raw_con;

    // Register some basic functions as a proof of concept
    // Address functions
    register_varchar_fn(con_handle, "fakeit_address_street", fakeit::address::street)?;
    register_varchar_fn(con_handle, "fakeit_address_city", fakeit::address::city)?;
    register_varchar_fn(con_handle, "fakeit_address_state", fakeit::address::state)?;
    register_varchar_fn(con_handle, "fakeit_address_zip", fakeit::address::zip)?;
    register_varchar_fn(con_handle, "fakeit_address_country", fakeit::address::country)?;

    // Name functions
    register_varchar_fn(con_handle, "fakeit_name_first", fakeit::name::first)?;
    register_varchar_fn(con_handle, "fakeit_name_last", fakeit::name::last)?;
    register_varchar_fn(con_handle, "fakeit_name_full", fakeit::name::full)?;

    // Company functions
    register_varchar_fn(con_handle, "fakeit_company_company", fakeit::company::company)?;
    register_varchar_fn(con_handle, "fakeit_company_bs", fakeit::company::bs)?;

    // Contact functions
    register_varchar_fn(con_handle, "fakeit_contact_email", fakeit::contact::email)?;
    register_varchar_fn(con_handle, "fakeit_contact_phone", fakeit::contact::phone)?;

    // Internet functions
    register_varchar_fn(con_handle, "fakeit_internet_ipv4_address", fakeit::internet::ipv4_address)?;
    register_varchar_fn(con_handle, "fakeit_internet_domain_name", fakeit::internet::domain_name)?;
    register_varchar_fn(con_handle, "fakeit_internet_username", fakeit::internet::username)?;

    // UUID functions
    register_varchar_fn(con_handle, "fakeit_uuid_v1", fakeit::unique::uuid_v1)?;
    register_varchar_fn(con_handle, "fakeit_uuid_v4", fakeit::unique::uuid_v4)?;

    // More Address functions
    register_varchar_fn(con_handle, "fakeit_address_street_number", fakeit::address::street_number)?;
    register_varchar_fn(con_handle, "fakeit_address_street_prefix", fakeit::address::street_prefix)?;
    register_varchar_fn(con_handle, "fakeit_address_street_name", fakeit::address::street_name)?;
    register_varchar_fn(con_handle, "fakeit_address_street_suffix", fakeit::address::street_suffix)?;
    register_varchar_fn(con_handle, "fakeit_address_state_abr", fakeit::address::state_abr)?;
    register_varchar_fn(con_handle, "fakeit_address_country_abr", fakeit::address::country_abr)?;

    // Animal functions
    register_varchar_fn(con_handle, "fakeit_animal_pet_name", fakeit::animal::pet_name)?;
    register_varchar_fn(con_handle, "fakeit_animal_animal", fakeit::animal::animal)?;
    register_varchar_fn(con_handle, "fakeit_animal_farm", fakeit::animal::farm)?;
    register_varchar_fn(con_handle, "fakeit_animal_cat", fakeit::animal::cat)?;
    register_varchar_fn(con_handle, "fakeit_animal_dog", fakeit::animal::dog)?;

    // Beer functions
    register_varchar_fn(con_handle, "fakeit_beer_name", fakeit::beer::name)?;
    register_varchar_fn(con_handle, "fakeit_beer_style", fakeit::beer::style)?;
    register_varchar_fn(con_handle, "fakeit_beer_hop", fakeit::beer::hop)?;
    register_varchar_fn(con_handle, "fakeit_beer_yeast", fakeit::beer::yeast)?;
    register_varchar_fn(con_handle, "fakeit_beer_malt", fakeit::beer::malt)?;
    register_varchar_fn(con_handle, "fakeit_beer_ibu", fakeit::beer::ibu)?;
    register_varchar_fn(con_handle, "fakeit_beer_alcohol", fakeit::beer::alcohol)?;
    register_varchar_fn(con_handle, "fakeit_beer_blg", fakeit::beer::blg)?;

    // Color functions
    register_varchar_fn(con_handle, "fakeit_color_full", fakeit::color::full)?;
    register_varchar_fn(con_handle, "fakeit_color_hex", fakeit::color::hex)?;
    register_varchar_fn(con_handle, "fakeit_color_safe", fakeit::color::safe)?;
    register_varchar_fn(con_handle, "fakeit_color_rgb", || {
        let rgb = fakeit::color::rgb();
        format!("[{}, {}, {}]", rgb[0], rgb[1], rgb[2])
    })?;

    // More Company functions
    register_varchar_fn(con_handle, "fakeit_company_company_suffix", fakeit::company::company_suffix)?;
    register_varchar_fn(con_handle, "fakeit_company_buzzword", fakeit::company::buzzword)?;

    // Contact functions
    register_varchar_fn(con_handle, "fakeit_contact_phone_formatted", fakeit::contact::phone_formatted)?;

    // Currency functions
    register_varchar_fn(con_handle, "fakeit_currency_short", fakeit::currency::short)?;
    register_varchar_fn(con_handle, "fakeit_currency_long", fakeit::currency::long)?;

    // DateTime functions
    register_varchar_fn(con_handle, "fakeit_datetime_month", fakeit::datetime::month)?;
    register_varchar_fn(con_handle, "fakeit_datetime_day", fakeit::datetime::day)?;
    register_varchar_fn(con_handle, "fakeit_datetime_week_day", fakeit::datetime::week_day)?;
    register_varchar_fn(con_handle, "fakeit_datetime_timezone", fakeit::datetime::timezone)?;
    register_varchar_fn(con_handle, "fakeit_datetime_timezone_full", fakeit::datetime::timezone_full)?;
    register_varchar_fn(con_handle, "fakeit_datetime_timezone_abv", fakeit::datetime::timezone_abv)?;

    // File functions
    register_varchar_fn(con_handle, "fakeit_file_extension", fakeit::file::extension)?;
    register_varchar_fn(con_handle, "fakeit_file_mime_type", fakeit::file::mime_type)?;

    // Hacker functions
    register_varchar_fn(con_handle, "fakeit_hacker_phrase", fakeit::hacker::phrase)?;
    register_varchar_fn(con_handle, "fakeit_hacker_abbreviation", fakeit::hacker::abbreviation)?;
    register_varchar_fn(con_handle, "fakeit_hacker_adjective", fakeit::hacker::adjective)?;
    register_varchar_fn(con_handle, "fakeit_hacker_noun", fakeit::hacker::noun)?;
    register_varchar_fn(con_handle, "fakeit_hacker_verb", fakeit::hacker::verb)?;
    register_varchar_fn(con_handle, "fakeit_hacker_ingverb", fakeit::hacker::ingverb)?;

    // Hipster functions
    register_varchar_fn(con_handle, "fakeit_hipster_word", fakeit::hipster::word)?;
    register_varchar_fn(con_handle, "fakeit_hipster_sentence", || fakeit::hipster::sentence(5))?;
    register_varchar_fn(con_handle, "fakeit_hipster_paragraph", || {
        fakeit::hipster::paragraph(3, 5, 10, String::from(" "))
    })?;

    // Image functions
    register_varchar_fn(con_handle, "fakeit_image_url", || fakeit::image::url(640, 480))?;

    // Internet functions
    register_varchar_fn(con_handle, "fakeit_internet_ipv6_address", fakeit::internet::ipv6_address)?;
    register_varchar_fn(con_handle, "fakeit_internet_domain_suffix", fakeit::internet::domain_suffix)?;
    register_varchar_fn(con_handle, "fakeit_internet_http_method", fakeit::internet::http_method)?;
    register_varchar_fn(con_handle, "fakeit_internet_mac_address", fakeit::internet::mac_address)?;

    // Job functions
    register_varchar_fn(con_handle, "fakeit_job_title", fakeit::job::title)?;
    register_varchar_fn(con_handle, "fakeit_job_descriptor", fakeit::job::descriptor)?;
    register_varchar_fn(con_handle, "fakeit_job_level", fakeit::job::level)?;

    // Language functions
    register_varchar_fn(con_handle, "fakeit_language_random", fakeit::language::random)?;
    register_varchar_fn(con_handle, "fakeit_language_abbreviation", fakeit::language::abbreviation)?;
    register_varchar_fn(con_handle, "fakeit_language_programming", fakeit::language::programming)?;

    // Log Level functions
    register_varchar_fn(con_handle, "fakeit_log_level_general", fakeit::log_level::general)?;
    register_varchar_fn(con_handle, "fakeit_log_level_syslog", fakeit::log_level::syslog)?;
    register_varchar_fn(con_handle, "fakeit_log_level_apache", fakeit::log_level::apache)?;

    // Name functions
    register_varchar_fn(con_handle, "fakeit_name_prefix", fakeit::name::prefix)?;
    register_varchar_fn(con_handle, "fakeit_name_suffix", fakeit::name::suffix)?;

    // Password functions
    register_varchar_fn(con_handle, "fakeit_password_generate", || {
        fakeit::password::generate(true, true, true, 16)
    })?;

    // Payment functions
    register_varchar_fn(con_handle, "fakeit_payment_credit_card_type", fakeit::payment::credit_card_type)?;
    register_varchar_fn(con_handle, "fakeit_payment_credit_card_number", fakeit::payment::credit_card_number)?;
    register_varchar_fn(con_handle, "fakeit_payment_credit_card_exp", fakeit::payment::credit_card_exp)?;
    register_varchar_fn(con_handle, "fakeit_payment_credit_card_cvv", fakeit::payment::credit_card_cvv)?;

    // Person functions
    register_varchar_fn(con_handle, "fakeit_person_ssn", fakeit::person::ssn)?;
    register_varchar_fn(con_handle, "fakeit_person_gender", fakeit::person::gender)?;

    // User Agent functions
    register_varchar_fn(con_handle, "fakeit_user_agent_chrome", fakeit::user_agent::chrome)?;
    register_varchar_fn(con_handle, "fakeit_user_agent_firefox", fakeit::user_agent::firefox)?;
    register_varchar_fn(con_handle, "fakeit_user_agent_safari", fakeit::user_agent::safari)?;
    register_varchar_fn(con_handle, "fakeit_user_agent_opera", fakeit::user_agent::opera)?;
    register_varchar_fn(con_handle, "fakeit_user_agent_linux_platform_token", fakeit::user_agent::linux_platform_token)?;
    register_varchar_fn(con_handle, "fakeit_user_agent_mac_platform_token", fakeit::user_agent::mac_platform_token)?;
    register_varchar_fn(con_handle, "fakeit_user_agent_windows_platform_token", fakeit::user_agent::windows_platform_token)?;
    register_varchar_fn(con_handle, "fakeit_user_agent_random_platform", fakeit::user_agent::random_platform)?;

    // Vehicle functions
    register_varchar_fn(con_handle, "fakeit_vehicle_vehicle_type", fakeit::vehicle::vehicle_type)?;
    register_varchar_fn(con_handle, "fakeit_vehicle_fuel", fakeit::vehicle::fuel)?;
    register_varchar_fn(con_handle, "fakeit_vehicle_transmission_gear", fakeit::vehicle::transmission_gear)?;
    register_varchar_fn(con_handle, "fakeit_vehicle_car_maker", fakeit::vehicle::car_maker)?;
    register_varchar_fn(con_handle, "fakeit_vehicle_car_model", fakeit::vehicle::car_model)?;

    // Words functions
    register_varchar_fn(con_handle, "fakeit_words_word", fakeit::words::word)?;
    register_varchar_fn(con_handle, "fakeit_words_sentence", || fakeit::words::sentence(10))?;
    register_varchar_fn(con_handle, "fakeit_words_paragraph", || {
        fakeit::words::paragraph(3, 5, 10, String::from(" "))
    })?;
    register_varchar_fn(con_handle, "fakeit_words_question", fakeit::words::question)?;
    register_varchar_fn(con_handle, "fakeit_words_quote", fakeit::words::quote)?;

    Ok(())
}
