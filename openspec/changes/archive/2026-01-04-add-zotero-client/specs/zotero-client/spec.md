## ADDED Requirements

### Requirement: Zotero Client Connection
The system SHALL provide a client that connects to the Zotero MCP plugin HTTP API.

#### Scenario: Default connection to local Zotero
- **WHEN** a client is created with default settings
- **THEN** it connects to `http://localhost:23119/mcp`

#### Scenario: Custom base URL
- **WHEN** a client is created with a custom base URL
- **THEN** it uses that URL for all API requests

### Requirement: Plugin Health Check
The system SHALL provide a method to check if the Zotero MCP plugin is active.

#### Scenario: Plugin is running
- **WHEN** the ping method is called
- **AND** Zotero is running with the MCP plugin
- **THEN** it returns plugin version and Zotero version

#### Scenario: Plugin is not available
- **WHEN** the ping method is called
- **AND** Zotero is not running or plugin is not installed
- **THEN** it returns a connection error

### Requirement: Item Search
The system SHALL provide methods to search and list Zotero library items.

#### Scenario: Search items by query
- **WHEN** `search_items` is called with a query string
- **THEN** it returns items matching the query (title, authors, etc.)

#### Scenario: List all items
- **WHEN** `list_items` is called with a limit
- **THEN** it returns up to that many top-level items from the library

#### Scenario: Search with no results
- **WHEN** `search_items` is called with a query that matches nothing
- **THEN** it returns an empty list

### Requirement: Item Retrieval
The system SHALL provide a method to retrieve a specific item by its key.

#### Scenario: Get existing item
- **WHEN** `get_item` is called with a valid item key
- **THEN** it returns the full item data including metadata and attachments

#### Scenario: Get non-existent item
- **WHEN** `get_item` is called with an invalid key
- **THEN** it returns a not found error

### Requirement: Children Retrieval
The system SHALL provide a method to retrieve child items (attachments, notes, annotations).

#### Scenario: Get attachments for regular item
- **WHEN** `get_children` is called for a regular item (book, article, etc.)
- **THEN** it returns attachments (PDFs, etc.) and notes

#### Scenario: Get annotations for PDF attachment
- **WHEN** `get_children` is called for a PDF attachment item
- **THEN** it returns all annotations on that PDF

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

#### Scenario: Create annotation on invalid parent
- **WHEN** `create_annotation` is called with a non-existent parent key
- **THEN** it returns a not found error

### Requirement: Error Handling
The system SHALL use typed errors for all failure modes.

#### Scenario: Connection error
- **WHEN** the Zotero server is unreachable
- **THEN** a `ConnectionError` is returned with details

#### Scenario: API error response
- **WHEN** the API returns an error status code
- **THEN** an `ApiError` is returned with the status and message

#### Scenario: Invalid response format
- **WHEN** the API returns malformed JSON
- **THEN** a `ParseError` is returned
