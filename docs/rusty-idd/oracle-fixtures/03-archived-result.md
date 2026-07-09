# Export Capability

## Purpose
File export capability for widget data.
## Requirements
### Requirement: CSV export
The system SHALL export widget data as CSV.

#### Scenario: Successful CSV export
The user requests a CSV export and receives a valid file.
# widget-export Specification

## Purpose
Defines how users export widget data.
## Requirements
### Requirement: CSV export
The system SHALL allow users to export widget data as CSV.

#### Scenario: Successful CSV export
- **GIVEN** a user has saved widgets
- **WHEN** the user exports their widgets as CSV
- **THEN** the system provides a CSV file containing the widgets

### Requirement: Export rate limit
The system SHALL limit exports to 20 per hour.

#### Scenario: Under the limit
The user has exported 19 times without error.

#### Scenario: Over the limit
The user is blocked after 20 exports.

### Requirement: Exported file naming
The system SHALL include a datestamp in the exported filename.

#### Scenario: Datestamp in filename
The exported file name contains today's date.

### Requirement: JSON export
The system SHALL export widget data as JSON.

#### Scenario: Successful JSON export
The user requests a JSON export and receives a valid file.
#### Scenario: Within limit
- **GIVEN** a user has exported 19 times this hour
- **WHEN** the user requests another export
- **THEN** the export succeeds

#### Scenario: Over limit
- **GIVEN** a user has exported 20 times this hour
- **WHEN** the user requests another export
- **THEN** the system rejects the export with a rate-limit error

### Requirement: Exported file naming
The system SHALL name exported files using the widget set name.

#### Scenario: Filename uses set name
- **GIVEN** a widget set named "alpha"
- **WHEN** the user exports it
- **THEN** the file is named "alpha.csv"

### Requirement: JSON export
The system SHALL allow users to export widget data as JSON.

#### Scenario: Successful JSON export
- **GIVEN** a user has saved widgets
- **WHEN** the user exports their widgets as JSON
- **THEN** the system provides a JSON file containing the widgets

