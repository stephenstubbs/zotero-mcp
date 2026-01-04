/**
 * MCP Zotero API Plugin
 * 
 * This plugin exposes HTTP endpoints for external tools (like MCP servers)
 * to create annotations and modify Zotero items while Zotero is running.
 * 
 * Endpoints:
 *   GET  /mcp/ping              - Check if the plugin is active
 *   POST /mcp/annotations       - Create a new annotation
 *   POST /mcp/items             - Get item details by key
 *   POST /mcp/search            - Search for items
 *   POST /mcp/children          - Get child items
 */

var MCP_Zotero;

function log(msg) {
    Zotero.debug("[MCP-Zotero] " + msg);
}

function install(data, reason) {
    log("Plugin installed");
}

function uninstall(data, reason) {
    log("Plugin uninstalled");
}

async function startup({ id, version, rootURI }, reason) {
    log("Starting MCP Zotero API plugin v" + version);
    
    // Wait for Zotero to be ready
    await Zotero.uiReadyPromise;
    
    // Initialize the MCP endpoints
    MCP_Zotero = {
        id,
        version,
        rootURI,
        endpoints: {}
    };
    
    // Register HTTP endpoints
    registerEndpoints();
    
    log("MCP Zotero API plugin started successfully");
}

function shutdown({ id, version, rootURI }, reason) {
    log("Shutting down MCP Zotero API plugin");
    
    // Unregister endpoints
    if (MCP_Zotero && MCP_Zotero.endpoints) {
        for (let path in MCP_Zotero.endpoints) {
            try {
                delete Zotero.Server.Endpoints[path];
                log("Unregistered endpoint: " + path);
            } catch (e) {
                log("Error unregistering endpoint " + path + ": " + e);
            }
        }
    }
    
    MCP_Zotero = null;
    log("MCP Zotero API plugin shut down");
}

function registerEndpoints() {
    // Ping endpoint - check if plugin is active
    registerEndpoint("/mcp/ping", {
        supportedMethods: ["GET"],
        supportedDataTypes: ["application/json"],
        init: function(data, sendResponseCallback) {
            sendResponseCallback(200, "application/json", JSON.stringify({
                status: "ok",
                plugin: "mcp-zotero-api",
                version: MCP_Zotero.version,
                zoteroVersion: Zotero.version
            }));
        }
    });
    
    // Create annotation endpoint
    registerEndpoint("/mcp/annotations", {
        supportedMethods: ["POST"],
        supportedDataTypes: ["application/json", "text/plain"],
        init: async function(requestData, sendResponseCallback) {
            try {
                let data;
                // Zotero already parses JSON for us
                if (typeof requestData === 'object' && requestData !== null) {
                    data = requestData;
                } else if (typeof requestData === 'string') {
                    try {
                        if (requestData.startsWith('%')) {
                            requestData = decodeURIComponent(requestData);
                        }
                        data = JSON.parse(requestData);
                    } catch (e) {
                        sendResponseCallback(400, "application/json", JSON.stringify({
                            error: "Invalid JSON",
                            message: e.message
                        }));
                        return;
                    }
                } else {
                    data = {};
                }
                
                // Validate required fields
                if (!data.parentItemKey) {
                    sendResponseCallback(400, "application/json", JSON.stringify({
                        error: "Missing required field: parentItemKey"
                    }));
                    return;
                }
                
                if (!data.annotationType) {
                    data.annotationType = "highlight";
                }
                
                // Find the parent item (should be a PDF attachment)
                let parentItem = await Zotero.Items.getByLibraryAndKeyAsync(
                    Zotero.Libraries.userLibraryID,
                    data.parentItemKey
                );
                
                if (!parentItem) {
                    sendResponseCallback(404, "application/json", JSON.stringify({
                        error: "Parent item not found",
                        key: data.parentItemKey
                    }));
                    return;
                }
                
                // Create the annotation
                let annotation = new Zotero.Item('annotation');
                annotation.libraryID = parentItem.libraryID;
                annotation.parentID = parentItem.id;
                
                // Set annotation properties
                annotation.annotationType = data.annotationType;
                
                if (data.text) {
                    annotation.annotationText = data.text;
                }
                
                if (data.comment) {
                    annotation.annotationComment = data.comment;
                }
                
                if (data.color) {
                    annotation.annotationColor = data.color;
                } else {
                    annotation.annotationColor = "#ffd400"; // Default yellow
                }
                
                if (data.pageLabel) {
                    annotation.annotationPageLabel = String(data.pageLabel);
                }
                
                // sortIndex is required - generate one if not provided
                // Format: NNNNN|NNNNNN|NNNNN (pageIndex|charOffset|charLength in padded format)
                if (data.sortIndex) {
                    annotation.annotationSortIndex = data.sortIndex;
                } else {
                    // Generate a default sortIndex based on page
                    let pageIdx = 0;
                    if (data.position && typeof data.position === 'object' && data.position.pageIndex !== undefined) {
                        pageIdx = data.position.pageIndex;
                    } else if (data.pageLabel) {
                        pageIdx = parseInt(data.pageLabel) - 1 || 0;
                    }
                    // Format: 5 digits for page | 6 digits for offset | 5 digits
                    annotation.annotationSortIndex = String(pageIdx).padStart(5, '0') + "|000000|00000";
                }
                
                if (data.position) {
                    // Position should be a JSON string or object
                    if (typeof data.position === 'object') {
                        annotation.annotationPosition = JSON.stringify(data.position);
                    } else {
                        annotation.annotationPosition = data.position;
                    }
                }
                
                // Save the annotation
                await annotation.saveTx();
                
                log("Created annotation: " + annotation.key + " on item " + data.parentItemKey);
                
                sendResponseCallback(201, "application/json", JSON.stringify({
                    success: true,
                    annotation: {
                        id: annotation.id,
                        key: annotation.key,
                        parentItemKey: data.parentItemKey,
                        type: annotation.annotationType,
                        text: annotation.annotationText,
                        color: annotation.annotationColor,
                        pageLabel: annotation.annotationPageLabel
                    }
                }));
                
            } catch (e) {
                log("Error creating annotation: " + e);
                sendResponseCallback(500, "application/json", JSON.stringify({
                    error: "Internal error",
                    message: e.message
                }));
            }
        }
    });
    
    // Get item by key endpoint (POST to pass JSON body)
    registerEndpoint("/mcp/item", {
        supportedMethods: ["POST"],
        supportedDataTypes: ["application/json", "text/plain"],
        init: async function(requestData, sendResponseCallback) {
            try {
                let data;
                if (typeof requestData === 'object' && requestData !== null) {
                    data = requestData;
                } else if (typeof requestData === 'string') {
                    try {
                        data = JSON.parse(requestData);
                    } catch (e) {
                        sendResponseCallback(400, "application/json", JSON.stringify({
                            error: "Invalid JSON",
                            message: e.message
                        }));
                        return;
                    }
                } else {
                    data = {};
                }
                
                let key = data.key;
                
                if (!key) {
                    sendResponseCallback(400, "application/json", JSON.stringify({
                        error: "Missing required field: key"
                    }));
                    return;
                }
                
                let item = await Zotero.Items.getByLibraryAndKeyAsync(
                    Zotero.Libraries.userLibraryID,
                    key
                );
                
                if (!item) {
                    sendResponseCallback(404, "application/json", JSON.stringify({
                        error: "Item not found",
                        key: key
                    }));
                    return;
                }
                
                let itemData = {
                    id: item.id,
                    key: item.key,
                    itemType: item.itemType,
                    title: item.getField('title'),
                    dateAdded: item.dateAdded,
                    dateModified: item.dateModified
                };
                
                // Add type-specific fields
                if (item.isRegularItem()) {
                    itemData.creators = item.getCreatorsJSON();
                    itemData.date = item.getField('date');
                    itemData.abstractNote = item.getField('abstractNote');
                    itemData.url = item.getField('url');
                    itemData.DOI = item.getField('DOI');
                    itemData.extra = item.getField('extra');
                    
                    // Get attachments
                    let attachmentIDs = item.getAttachments();
                    itemData.attachments = [];
                    for (let attID of attachmentIDs) {
                        let att = await Zotero.Items.getAsync(attID);
                        itemData.attachments.push({
                            id: att.id,
                            key: att.key,
                            title: att.getField('title'),
                            contentType: att.attachmentContentType,
                            path: att.getFilePath()
                        });
                    }
                }
                
                if (item.isAttachment()) {
                    itemData.contentType = item.attachmentContentType;
                    itemData.path = item.getFilePath();
                    itemData.parentItemID = item.parentItemID;
                }
                
                sendResponseCallback(200, "application/json", JSON.stringify(itemData));
                
            } catch (e) {
                log("Error getting item: " + e);
                sendResponseCallback(500, "application/json", JSON.stringify({
                    error: "Internal error",
                    message: e.message
                }));
            }
        }
    });
    
    // Search items endpoint (POST to pass JSON body)
    registerEndpoint("/mcp/search", {
        supportedMethods: ["POST"],
        supportedDataTypes: ["application/json", "text/plain"],
        init: async function(requestData, sendResponseCallback) {
            try {
                log("search received type: " + typeof requestData);
                let data;
                // Zotero already parses JSON for us
                if (typeof requestData === 'object' && requestData !== null) {
                    data = requestData;
                } else if (typeof requestData === 'string') {
                    try {
                        if (requestData.startsWith('%')) {
                            requestData = decodeURIComponent(requestData);
                        }
                        data = JSON.parse(requestData);
                    } catch (e) {
                        sendResponseCallback(400, "application/json", JSON.stringify({
                            error: "Invalid JSON",
                            message: e.message
                        }));
                        return;
                    }
                } else {
                    data = {};
                }
                
                let query = data.query || data.q;
                let limit = parseInt(data.limit) || 25;
                
                if (!query) {
                    sendResponseCallback(400, "application/json", JSON.stringify({
                        error: "Missing required field: query"
                    }));
                    return;
                }
                
                let s = new Zotero.Search();
                s.libraryID = Zotero.Libraries.userLibraryID;
                s.addCondition('quicksearch-everything', 'contains', query);
                
                let ids = await s.search();
                ids = ids.slice(0, limit);
                
                let items = await Zotero.Items.getAsync(ids);
                let results = [];
                
                for (let item of items) {
                    if (item.isRegularItem()) {
                        results.push({
                            id: item.id,
                            key: item.key,
                            itemType: item.itemType,
                            title: item.getField('title'),
                            creators: item.getCreatorsJSON(),
                            date: item.getField('date'),
                            extra: item.getField('extra')
                        });
                    }
                }
                
                sendResponseCallback(200, "application/json", JSON.stringify({
                    results: results,
                    total: results.length
                }));
                
            } catch (e) {
                log("Error searching items: " + e);
                sendResponseCallback(500, "application/json", JSON.stringify({
                    error: "Internal error",
                    message: e.message
                }));
            }
        }
    });
    
    // Get item children (attachments, notes, annotations)
    registerEndpoint("/mcp/children", {
        supportedMethods: ["POST"],
        supportedDataTypes: ["application/json", "text/plain"],
        init: async function(requestData, sendResponseCallback) {
            try {
                let data;
                if (typeof requestData === 'object' && requestData !== null) {
                    data = requestData;
                } else if (typeof requestData === 'string') {
                    try {
                        data = JSON.parse(requestData);
                    } catch (e) {
                        sendResponseCallback(400, "application/json", JSON.stringify({
                            error: "Invalid JSON",
                            message: e.message
                        }));
                        return;
                    }
                } else {
                    data = {};
                }
                
                let key = data.key;
                
                if (!key) {
                    sendResponseCallback(400, "application/json", JSON.stringify({
                        error: "Missing required field: key"
                    }));
                    return;
                }
                
                let item = await Zotero.Items.getByLibraryAndKeyAsync(
                    Zotero.Libraries.userLibraryID,
                    key
                );
                
                if (!item) {
                    sendResponseCallback(404, "application/json", JSON.stringify({
                        error: "Item not found",
                        key: key
                    }));
                    return;
                }
                
                let children = [];
                
                // Get attachments
                if (item.isRegularItem()) {
                    let attachmentIDs = item.getAttachments();
                    for (let attID of attachmentIDs) {
                        let att = await Zotero.Items.getAsync(attID);
                        children.push({
                            id: att.id,
                            key: att.key,
                            itemType: 'attachment',
                            title: att.getField('title'),
                            contentType: att.attachmentContentType,
                            path: att.getFilePath()
                        });
                    }
                    
                    // Get notes
                    let noteIDs = item.getNotes();
                    for (let noteID of noteIDs) {
                        let note = await Zotero.Items.getAsync(noteID);
                        children.push({
                            id: note.id,
                            key: note.key,
                            itemType: 'note',
                            note: note.getNote()
                        });
                    }
                }
                
                // If it's an attachment, get annotations
                if (item.isAttachment()) {
                    let annotations = item.getAnnotations();
                    for (let ann of annotations) {
                        children.push({
                            id: ann.id,
                            key: ann.key,
                            itemType: 'annotation',
                            annotationType: ann.annotationType,
                            text: ann.annotationText,
                            comment: ann.annotationComment,
                            color: ann.annotationColor,
                            pageLabel: ann.annotationPageLabel,
                            sortIndex: ann.annotationSortIndex,
                            position: ann.annotationPosition
                        });
                    }
                }
                
                sendResponseCallback(200, "application/json", JSON.stringify({
                    parentKey: key,
                    children: children
                }));
                
            } catch (e) {
                log("Error getting children: " + e);
                sendResponseCallback(500, "application/json", JSON.stringify({
                    error: "Internal error",
                    message: e.message
                }));
            }
        }
    });
    
    // Get all top-level items
    registerEndpoint("/mcp/items", {
        supportedMethods: ["POST"],
        supportedDataTypes: ["application/json", "text/plain"],
        init: async function(requestData, sendResponseCallback) {
            try {
                let data = {};
                if (typeof requestData === 'object' && requestData !== null) {
                    data = requestData;
                } else if (typeof requestData === 'string' && requestData) {
                    try {
                        data = JSON.parse(requestData);
                    } catch (e) {
                        // Ignore parse errors, use defaults
                    }
                }
                
                let limit = parseInt(data.limit) || 50;
                
                let s = new Zotero.Search();
                s.libraryID = Zotero.Libraries.userLibraryID;
                s.addCondition('itemType', 'isNot', 'attachment');
                s.addCondition('itemType', 'isNot', 'note');
                s.addCondition('itemType', 'isNot', 'annotation');
                
                let ids = await s.search();
                ids = ids.slice(0, limit);
                
                let items = await Zotero.Items.getAsync(ids);
                let results = [];
                
                for (let item of items) {
                    results.push({
                        id: item.id,
                        key: item.key,
                        itemType: item.itemType,
                        title: item.getField('title'),
                        creators: item.getCreatorsJSON(),
                        date: item.getField('date'),
                        extra: item.getField('extra')
                    });
                }
                
                sendResponseCallback(200, "application/json", JSON.stringify({
                    items: results,
                    total: results.length
                }));
                
            } catch (e) {
                log("Error getting items: " + e);
                sendResponseCallback(500, "application/json", JSON.stringify({
                    error: "Internal error",
                    message: e.message
                }));
            }
        }
    });
    
    // Lookup item by BetterBibTeX citation key
    registerEndpoint("/mcp/citekey", {
        supportedMethods: ["POST"],
        supportedDataTypes: ["application/json", "text/plain"],
        init: async function(requestData, sendResponseCallback) {
            try {
                let data;
                if (typeof requestData === 'object' && requestData !== null) {
                    data = requestData;
                } else if (typeof requestData === 'string') {
                    try {
                        data = JSON.parse(requestData);
                    } catch (e) {
                        sendResponseCallback(400, "application/json", JSON.stringify({
                            error: "Invalid JSON",
                            message: e.message
                        }));
                        return;
                    }
                } else {
                    data = {};
                }
                
                let citekey = data.citekey;
                
                if (!citekey) {
                    sendResponseCallback(400, "application/json", JSON.stringify({
                        error: "Missing required field: citekey"
                    }));
                    return;
                }
                
                // Try to use BetterBibTeX API if available
                let item = null;
                
                if (typeof Zotero.BetterBibTeX !== 'undefined') {
                    // BetterBibTeX is installed - use its API
                    try {
                        let result = await Zotero.BetterBibTeX.KeyManager.keys.findOne({ citekey: citekey });
                        if (result && result.itemID) {
                            item = await Zotero.Items.getAsync(result.itemID);
                        }
                    } catch (e) {
                        log("BetterBibTeX lookup failed: " + e);
                    }
                }
                
                // Fallback: search in extra field
                if (!item) {
                    let s = new Zotero.Search();
                    s.libraryID = Zotero.Libraries.userLibraryID;
                    s.addCondition('itemType', 'isNot', 'attachment');
                    s.addCondition('itemType', 'isNot', 'note');
                    s.addCondition('itemType', 'isNot', 'annotation');
                    
                    let ids = await s.search();
                    let items = await Zotero.Items.getAsync(ids);
                    
                    for (let it of items) {
                        let extra = it.getField('extra') || '';
                        if (extra.includes('Citation Key: ' + citekey) || 
                            extra.toLowerCase().includes('citekey: ' + citekey.toLowerCase())) {
                            item = it;
                            break;
                        }
                    }
                }
                
                if (!item) {
                    sendResponseCallback(404, "application/json", JSON.stringify({
                        error: "Item not found for citekey",
                        citekey: citekey
                    }));
                    return;
                }
                
                // Build response with item data
                let itemData = {
                    id: item.id,
                    key: item.key,
                    itemType: item.itemType,
                    title: item.getField('title'),
                    creators: item.getCreatorsJSON(),
                    date: item.getField('date'),
                    extra: item.getField('extra'),
                    citekey: citekey
                };
                
                // Get PDF attachments
                let attachmentIDs = item.getAttachments();
                itemData.attachments = [];
                for (let attID of attachmentIDs) {
                    let att = await Zotero.Items.getAsync(attID);
                    if (att.attachmentContentType === 'application/pdf') {
                        itemData.attachments.push({
                            id: att.id,
                            key: att.key,
                            title: att.getField('title'),
                            contentType: att.attachmentContentType,
                            path: att.getFilePath()
                        });
                    }
                }
                
                sendResponseCallback(200, "application/json", JSON.stringify(itemData));
                
            } catch (e) {
                log("Error looking up citekey: " + e);
                sendResponseCallback(500, "application/json", JSON.stringify({
                    error: "Internal error",
                    message: e.message
                }));
            }
        }
    });
    
    log("Registered " + Object.keys(MCP_Zotero.endpoints).length + " MCP endpoints");
}

function registerEndpoint(path, handler) {
    Zotero.Server.Endpoints[path] = function() {};
    Zotero.Server.Endpoints[path].prototype = handler;
    MCP_Zotero.endpoints[path] = true;
    log("Registered endpoint: " + path);
}
