CREATE TABLE IF NOT EXISTS machines (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,                    
    max_x REAL NOT NULL,                    
    max_y REAL NOT NULL,                    
    is_active BOOLEAN DEFAULT 1             
);

CREATE TABLE IF NOT EXISTS sheets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    material TEXT NOT NULL,             
    thickness_mm REAL NOT NULL,         
    width_mm REAL NOT NULL,             
    height_mm REAL NOT NULL,            
    price_per_sheet REAL NOT NULL,      
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP 
);

CREATE TABLE IF NOT EXISTS cutting_tariffs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    material TEXT NOT NULL,             
    thickness_mm REAL NOT NULL,         
    price_per_meter REAL NOT NULL,      
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP 
);

CREATE TABLE IF NOT EXISTS inspection_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name TEXT NOT NULL,            
    file_hash TEXT,                    
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    verdict TEXT NOT NULL,              
    width_mm REAL,
    height_mm REAL,
    cut_length_mm REAL,
    estimated_cost REAL,                
    error_reason TEXT,                  
    matched_sheet_id INTEGER,           
    FOREIGN KEY (matched_sheet_id) REFERENCES sheets(id)
);