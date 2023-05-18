CREATE TABLE IF NOT EXISTS file_folder_link
(
    file_id   UUID REFERENCES file (id),
    folder_id UUID REFERENCES folder (id),
    PRIMARY KEY (file_id, folder_id)
);
