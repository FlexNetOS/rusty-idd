# Export Capability

## Purpose
File export capability for widget data.
# widget-export Specification

## Purpose
Defines how users export widget data.

## Requirements

### Requirement: CSV export
The system SHALL export widget data as CSV.

#### Scenario: Successful CSV export
The user requests a CSV export and receives a valid file.
The system SHALL allow users to export widget data as CSV.

#### Scenario: Successful CSV export
- **GIVEN** a user has saved widgets
- **WHEN** the user exports their widgets as CSV
- **THEN** the system provides a CSV file containing the widgets

### Requirement: Export rate limit
The system SHALL limit exports to 10 per hour.

#### Scenario: Under the limit
The user has exported 9 times without error.

### Requirement: Export filename
The system SHALL include a datestamp in the exported filename.

#### Scenario: Datestamp in filename
The exported file name contains today's date.

### Requirement: Legacy XML export
The system SHALL export widget data as XML.

#### Scenario: Successful XML export
The user requests an XML export and receives a valid file.
#### Scenario: Within limit
- **GIVEN** a user has exported 9 times this hour
- **WHEN** the user requests another export
- **THEN** the export succeeds

#### Scenario: Over limit
- **GIVEN** a user has exported 10 times this hour
- **WHEN** the user requests another export
- **THEN** the system rejects the export with a rate-limit error

### Requirement: Legacy XML export
The system SHALL allow users to export widget data as XML.

#### Scenario: XML export
- **GIVEN** a user has saved widgets
- **WHEN** the user exports as XML
- **THEN** the system provides an XML file

### Requirement: Export filename
The system SHALL name exported files using the widget set name.

#### Scenario: Filename uses set name
- **GIVEN** a widget set named "alpha"
- **WHEN** the user exports it
- **THEN** the file is named "alpha.csv"
