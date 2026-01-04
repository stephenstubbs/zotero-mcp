## ADDED Requirements

### Requirement: Area Annotation Type
The Zotero plugin SHALL support creating image/area annotations.

#### Scenario: Create image annotation via API
- **WHEN** the `/mcp/annotations` endpoint receives `annotationType: "image"`
- **THEN** it creates an area annotation in Zotero

#### Scenario: Area annotation position
- **WHEN** an image annotation request includes position with `rects`
- **THEN** the annotation is created at the specified rectangular region

#### Scenario: Area annotation without text
- **WHEN** an image annotation is created
- **THEN** it does not require a `text` field (unlike highlights)

### Requirement: Area Annotation Client Support
The Rust client SHALL support creating area annotations.

#### Scenario: Create area annotation request
- **WHEN** `CreateAreaAnnotationRequest` is constructed
- **THEN** it sets `annotationType` to "image"

#### Scenario: Area annotation with coordinates
- **WHEN** `create_area_annotation` is called with rect coordinates
- **THEN** the request includes the position with page index and rects

#### Scenario: Area annotation with color
- **WHEN** `create_area_annotation` is called with a color
- **THEN** the annotation is created with that color

#### Scenario: Area annotation with comment
- **WHEN** `create_area_annotation` is called with a comment
- **THEN** the annotation includes the comment text
