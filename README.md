# DuckDB Fakeit Extension

A DuckDB extension that provides fake data generation capabilities using the Rust [fakeit](https://crates.io/crates/fakeit) library. Generate realistic test data directly in your SQL queries with 120+ functions covering addresses, names, companies, internet data, and much more.

## Features

- **Pure Rust implementation** - No C++ code required
- **120+ fake data functions** - Comprehensive coverage of common data types
- **No external dependencies** - Everything is self-contained
- **CI/CD preconfigured** - Ready for automated builds and distribution
- **WebAssembly support** - Works in DuckDB-WASM environments

## Installation

### From community extensions repo

```sql
INSTALL fakeit FROM community;
LOAD fakeit;
```

### From Source

Clone the repo with submodules:

```shell
git clone --recurse-submodules https://github.com/tobilg/duckdb-fakeit.git
cd duckdb-fakeit
```

### Dependencies

- Rust toolchain
- Python 3.13 (for testing)
- Python3-venv
- [Make](https://www.gnu.org/software/make)
- Git

Installing dependencies:
- **Linux**: Available through your distro's package manager
- **macOS**: Install via [Homebrew](https://formulae.brew.sh/)
- **Windows**: Install via [Chocolatey](https://community.chocolatey.org/)

## Building

Configure the build environment:
```shell
make configure
```

Build debug version:
```shell
make debug
```

Build optimized release version:
```shell
make release
```

## Usage

Load the extension in DuckDB:

```sql
-- Start DuckDB in unsigned mode (required for local builds)
duckdb -unsigned

-- Load the extension
LOAD './build/release/fakeit.duckdb_extension';

-- Generate fake data
SELECT
    fakeit_name_full() as name,
    fakeit_contact_email() as email,
    fakeit_address_city() as city,
    fakeit_address_country() as country;
```

Example output:
```
┌─────────────────┬──────────────────────────┬──────────────┬───────────────┐
│      name       │          email           │     city     │    country    │
│     varchar     │         varchar          │   varchar    │    varchar    │
├─────────────────┼──────────────────────────┼──────────────┼───────────────┤
│ Jane Doe        │ jane.doe@example.com     │ New York     │ United States │
└─────────────────┴──────────────────────────┴──────────────┴───────────────┘
```

## Function Reference

All functions are prefixed with `fakeit_` and organized by category. Most functions take no parameters and return VARCHAR or appropriate data types.

### Address Functions

Generate address-related data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_address_street()` | VARCHAR | Full street address |
| `fakeit_address_street_number()` | VARCHAR | Street number |
| `fakeit_address_street_prefix()` | VARCHAR | Street prefix (e.g., "North", "East") |
| `fakeit_address_street_name()` | VARCHAR | Street name |
| `fakeit_address_street_suffix()` | VARCHAR | Street suffix (e.g., "Street", "Avenue") |
| `fakeit_address_city()` | VARCHAR | City name |
| `fakeit_address_state()` | VARCHAR | Full state name |
| `fakeit_address_state_abr()` | VARCHAR | State abbreviation |
| `fakeit_address_zip()` | VARCHAR | ZIP/postal code |
| `fakeit_address_country()` | VARCHAR | Country name |
| `fakeit_address_country_abr()` | VARCHAR | Country abbreviation |
| `fakeit_address_latitude()` | DOUBLE | Latitude coordinate (-90 to 90) |
| `fakeit_address_longitude()` | DOUBLE | Longitude coordinate (-180 to 180) |

**Range-limited coordinates:**
| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `fakeit_address_latitude_in_range(min, max)` | DOUBLE, DOUBLE | DOUBLE | Latitude within range |
| `fakeit_address_longitude_in_range(min, max)` | DOUBLE, DOUBLE | DOUBLE | Longitude within range |

### Name Functions

Generate person names.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_name_first()` | VARCHAR | First name |
| `fakeit_name_last()` | VARCHAR | Last name |
| `fakeit_name_full()` | VARCHAR | Full name (first + last) |
| `fakeit_name_prefix()` | VARCHAR | Name prefix (e.g., "Mr.", "Dr.") |
| `fakeit_name_suffix()` | VARCHAR | Name suffix (e.g., "Jr.", "III") |

### Contact Functions

Generate contact information.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_contact_email()` | VARCHAR | Email address |
| `fakeit_contact_phone()` | VARCHAR | Phone number (unformatted) |
| `fakeit_contact_phone_formatted()` | VARCHAR | Phone number (formatted) |

### Company Functions

Generate company-related data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_company_company()` | VARCHAR | Company name |
| `fakeit_company_company_suffix()` | VARCHAR | Company suffix (e.g., "Inc", "LLC") |
| `fakeit_company_buzzword()` | VARCHAR | Business buzzword |
| `fakeit_company_bs()` | VARCHAR | Business statement |

### Internet Functions

Generate internet-related data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_internet_ipv4_address()` | VARCHAR | IPv4 address |
| `fakeit_internet_ipv6_address()` | VARCHAR | IPv6 address |
| `fakeit_internet_domain_name()` | VARCHAR | Domain name |
| `fakeit_internet_domain_suffix()` | VARCHAR | Domain suffix (e.g., ".com", ".org") |
| `fakeit_internet_username()` | VARCHAR | Username |
| `fakeit_internet_mac_address()` | VARCHAR | MAC address |
| `fakeit_internet_http_method()` | VARCHAR | HTTP method (GET, POST, etc.) |

### UUID Functions

Generate universally unique identifiers.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_uuid_v1()` | VARCHAR | UUID version 1 |
| `fakeit_uuid_v4()` | VARCHAR | UUID version 4 (random) |

### Animal Functions

Generate animal-related data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_animal_pet_name()` | VARCHAR | Pet name |
| `fakeit_animal_animal()` | VARCHAR | Animal type |
| `fakeit_animal_farm()` | VARCHAR | Farm animal |
| `fakeit_animal_cat()` | VARCHAR | Cat breed |
| `fakeit_animal_dog()` | VARCHAR | Dog breed |

### Beer Functions

Generate beer-related data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_beer_name()` | VARCHAR | Beer name |
| `fakeit_beer_style()` | VARCHAR | Beer style |
| `fakeit_beer_hop()` | VARCHAR | Hop variety |
| `fakeit_beer_yeast()` | VARCHAR | Yeast type |
| `fakeit_beer_malt()` | VARCHAR | Malt variety |
| `fakeit_beer_ibu()` | VARCHAR | IBU (bitterness) value |
| `fakeit_beer_alcohol()` | VARCHAR | Alcohol percentage |
| `fakeit_beer_blg()` | VARCHAR | Plato scale value |

### Color Functions

Generate color data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_color_full()` | VARCHAR | Full color name |
| `fakeit_color_hex()` | VARCHAR | Hex color code |
| `fakeit_color_safe()` | VARCHAR | Web-safe color |
| `fakeit_color_rgb()` | VARCHAR | RGB values as "[R, G, B]" |

### Currency Functions

Generate currency-related data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_currency_short()` | VARCHAR | Currency code (e.g., "USD") |
| `fakeit_currency_long()` | VARCHAR | Currency name (e.g., "US Dollar") |
| `fakeit_currency_price()` | DOUBLE | Random price value |

### DateTime Functions

Generate date and time data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_datetime_month()` | VARCHAR | Month name |
| `fakeit_datetime_day()` | VARCHAR | Day of month |
| `fakeit_datetime_week_day()` | VARCHAR | Day of week |
| `fakeit_datetime_year()` | VARCHAR | Year |
| `fakeit_datetime_hour()` | VARCHAR | Hour |
| `fakeit_datetime_minute()` | VARCHAR | Minute |
| `fakeit_datetime_second()` | VARCHAR | Second |
| `fakeit_datetime_nanosecond()` | VARCHAR | Nanosecond |
| `fakeit_datetime_timezone()` | VARCHAR | Timezone name |
| `fakeit_datetime_timezone_full()` | VARCHAR | Full timezone name |
| `fakeit_datetime_timezone_abv()` | VARCHAR | Timezone abbreviation |
| `fakeit_datetime_timezone_offset()` | VARCHAR | Timezone offset |
| `fakeit_datetime_date()` | VARCHAR | Random date |

### File Functions

Generate file-related data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_file_extension()` | VARCHAR | File extension |
| `fakeit_file_mime_type()` | VARCHAR | MIME type |

### Hacker Functions

Generate tech/hacker terminology.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_hacker_phrase()` | VARCHAR | Hacker phrase |
| `fakeit_hacker_abbreviation()` | VARCHAR | Tech abbreviation |
| `fakeit_hacker_adjective()` | VARCHAR | Tech adjective |
| `fakeit_hacker_noun()` | VARCHAR | Tech noun |
| `fakeit_hacker_verb()` | VARCHAR | Tech verb |
| `fakeit_hacker_ingverb()` | VARCHAR | Tech verb ending in -ing |

### Hipster Functions

Generate hipster-style text.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_hipster_word()` | VARCHAR | Hipster word |
| `fakeit_hipster_sentence()` | VARCHAR | Hipster sentence (5 words) |
| `fakeit_hipster_paragraph()` | VARCHAR | Hipster paragraph |

### Image Functions

Generate image URLs.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_image_url()` | VARCHAR | Placeholder image URL (640x480) |

### Job Functions

Generate job-related data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_job_title()` | VARCHAR | Job title |
| `fakeit_job_descriptor()` | VARCHAR | Job descriptor |
| `fakeit_job_level()` | VARCHAR | Job level |

### Language Functions

Generate language data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_language_random()` | VARCHAR | Random language |
| `fakeit_language_abbreviation()` | VARCHAR | Language abbreviation |
| `fakeit_language_programming()` | VARCHAR | Programming language |

### Log Level Functions

Generate log levels.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_log_level_general()` | VARCHAR | General log level |
| `fakeit_log_level_syslog()` | VARCHAR | Syslog severity level |
| `fakeit_log_level_apache()` | VARCHAR | Apache log level |

### Password Functions

Generate passwords.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_password_generate()` | VARCHAR | Random password |

### Payment Functions

Generate payment card data (for testing only).

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_payment_credit_card_type()` | VARCHAR | Credit card type |
| `fakeit_payment_credit_card_number()` | VARCHAR | Credit card number |
| `fakeit_payment_credit_card_exp()` | VARCHAR | Expiration date |
| `fakeit_payment_credit_card_cvv()` | VARCHAR | CVV code |
| `fakeit_payment_credit_card_luhn_number()` | VARCHAR | Luhn-valid number |

### Person Functions

Generate person-related data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_person_ssn()` | VARCHAR | Social Security Number |
| `fakeit_person_gender()` | VARCHAR | Gender |

### Status Code Functions

Generate HTTP status codes.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_status_code_simple()` | BIGINT | Simple HTTP status code |
| `fakeit_status_code_general()` | BIGINT | General HTTP status code |

### User Agent Functions

Generate browser user agents.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_user_agent_chrome()` | VARCHAR | Chrome user agent |
| `fakeit_user_agent_firefox()` | VARCHAR | Firefox user agent |
| `fakeit_user_agent_safari()` | VARCHAR | Safari user agent |
| `fakeit_user_agent_opera()` | VARCHAR | Opera user agent |
| `fakeit_user_agent_random_platform()` | VARCHAR | Random platform token |
| `fakeit_user_agent_linux_platform_token()` | VARCHAR | Linux platform token |
| `fakeit_user_agent_mac_platform_token()` | VARCHAR | macOS platform token |
| `fakeit_user_agent_windows_platform_token()` | VARCHAR | Windows platform token |

### Vehicle Functions

Generate vehicle data.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_vehicle_vehicle_type()` | VARCHAR | Vehicle type |
| `fakeit_vehicle_car_maker()` | VARCHAR | Car manufacturer |
| `fakeit_vehicle_car_model()` | VARCHAR | Car model |
| `fakeit_vehicle_fuel()` | VARCHAR | Fuel type |
| `fakeit_vehicle_transmission_gear()` | VARCHAR | Transmission type |

### Words Functions

Generate random text.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_words_word()` | VARCHAR | Random word |
| `fakeit_words_sentence()` | VARCHAR | Random sentence |
| `fakeit_words_paragraph()` | VARCHAR | Random paragraph |
| `fakeit_words_question()` | VARCHAR | Random question |
| `fakeit_words_quote()` | VARCHAR | Random quote |

### Boolean Functions

Generate random boolean values.

| Function | Returns | Description |
|----------|---------|-------------|
| `fakeit_bool()` | BOOLEAN | Random true/false |

### Generator Functions

Template-based generation.

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `fakeit_generator_generate(template)` | VARCHAR | VARCHAR | Generate from template pattern |

Template syntax: Use `{category.function}` patterns (e.g., `{name.first} {name.last} lives in {address.city}`).

## Examples

### Generate Test Users

```sql
SELECT
    row_number() OVER () as id,
    fakeit_name_full() as name,
    fakeit_contact_email() as email,
    fakeit_contact_phone_formatted() as phone,
    fakeit_address_city() as city,
    fakeit_address_state() as state
FROM generate_series(1, 100);
```

### Generate E-commerce Orders

```sql
SELECT
    fakeit_uuid_v4() as order_id,
    fakeit_name_full() as customer_name,
    fakeit_company_company() as vendor,
    fakeit_currency_price() as amount,
    fakeit_payment_credit_card_type() as payment_method,
    fakeit_datetime_date() as order_date
FROM generate_series(1, 1000);
```

### Generate Log Entries

```sql
SELECT
    fakeit_datetime_date() as timestamp,
    fakeit_internet_ipv4_address() as ip_address,
    fakeit_log_level_apache() as log_level,
    fakeit_internet_http_method() as http_method,
    fakeit_status_code_general() as status_code,
    fakeit_user_agent_chrome() as user_agent
FROM generate_series(1, 10000);
```

### Single-Row Usage
```sql
SELECT
    fakeit_name_full() as name,
    fakeit_contact_email() as email,
    fakeit_address_city() as city,
    fakeit_address_country() as country;
```

### Generate GeoJSON Data

```sql
SELECT json_object(
    'type', 'Feature',
    'geometry', json_object(
        'type', 'Point',
        'coordinates', json_array(
            fakeit_address_longitude(),
            fakeit_address_latitude()
        )
    ),
    'properties', json_object(
        'name', fakeit_name_full(),
        'city', fakeit_address_city(),
        'country', fakeit_address_country()
    )
) as geojson
FROM generate_series(1, 50);
```

## Testing

Run tests with the debug build:
```shell
make test_debug
```

Run tests with the release build:
```shell
make test_release
```

### Version Switching

Test with different DuckDB versions:

```shell
make clean_all
DUCKDB_TEST_VERSION=v1.4.2 make configure
make debug
make test_debug
```

### Local Testing with Python 3.14+

Due to DuckDB's unstable C API (`USE_UNSTABLE_C_API=1`), local testing may encounter segmentation faults when using Python 3.14+, as pre-built wheels don't exist for DuckDB v1.4.2 with Python 3.14. The extension works perfectly in CI/CD and for end users.

**Workaround**: Use Python 3.13 for local testing. The Makefile is already configured to use Python 3.13.

### Windows Python 3.11

Extensions may fail to load on Windows with Python 3.11 with the error:
```
IO Error: Extension could not be loaded: The specified module could not be found
```
This is resolved by using Python 3.12 or later.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

See [LICENSE](LICENSE) file for details.

## Acknowledgments

Built using:
- [DuckDB](https://duckdb.org/) - An in-process SQL OLAP database
- [fakeit](https://crates.io/crates/fakeit) - Rust fake data generator
- [DuckDB Extension Template for Rust](https://github.com/duckdb/extension-template-rust)
