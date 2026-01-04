## ADDED Requirements

### Requirement: Semantic Highlight Colors
The system SHALL provide an enum of semantic highlight colors.

#### Scenario: Color enum variants
- **WHEN** a user needs to specify a highlight color
- **THEN** they can use `HighlightColor::Section1`, `HighlightColor::Positive`, etc.

#### Scenario: Color to hex conversion
- **WHEN** a `HighlightColor` variant is converted to a string
- **THEN** it returns the corresponding hex code

### Requirement: Area Annotation Creation
The system SHALL provide a method to create area/image annotations on PDF attachments.

#### Scenario: Create area annotation
- **WHEN** `create_area_annotation` is called with parent PDF key, page, and rect
- **THEN** it creates an image annotation in Zotero
- **AND** returns the created annotation with its key

#### Scenario: Create area annotation with comment
- **WHEN** `create_area_annotation` is called with a comment
- **THEN** the annotation includes the comment text

#### Scenario: Create area annotation with color
- **WHEN** `create_area_annotation` is called with a `HighlightColor`
- **THEN** the annotation uses that color's hex code

#### Scenario: Create area annotation on invalid parent
- **WHEN** `create_area_annotation` is called with a non-existent parent key
- **THEN** it returns a not found error

### Requirement: Area Annotation Request Type
The system SHALL provide a typed request for area annotations.

#### Scenario: Builder pattern
- **WHEN** `CreateAreaAnnotationRequest::new()` is called
- **THEN** it can be chained with `.with_comment()` and `.with_color()`

#### Scenario: Annotation type set correctly
- **WHEN** a `CreateAreaAnnotationRequest` is serialized
- **THEN** it includes `"annotationType": "image"`

## MODIFIED Requirements

### Requirement: Annotation Creation
The system SHALL provide a method to create highlight annotations on PDF attachments.

#### Scenario: Create highlight annotation
- **WHEN** `create_annotation` is called with parent PDF key, text, page, and position
- **THEN** it creates a highlight annotation in Zotero
- **AND** returns the created annotation with its key

#### Scenario: Create annotation with comment
- **WHEN** `create_annotation` is called with a comment
- **THEN** the annotation includes the comment text

#### Scenario: Create annotation with custom color
- **WHEN** `create_annotation` is called with a color (e.g., "#ff0000")
- **THEN** the annotation uses that color

#### Scenario: Create annotation with semantic color
- **WHEN** `create_annotation` is called with a `HighlightColor` variant
- **THEN** the annotation uses the corresponding hex color

#### Scenario: Create annotation on invalid parent
- **WHEN** `create_annotation` is called with a non-existent parent key
- **THEN** it returns a not found error
