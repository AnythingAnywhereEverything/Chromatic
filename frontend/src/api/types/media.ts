export interface Media {
    id: string // * media from media_data
    uploader_id?: string
    
    original_name?: string
    original_content_type?: string
    
    file_type: string

    flags: number
    media_objects: MediaObjects[]
    media_object_metadata: MediaObjectMetadata
    media_hls?: MediaHls
    media_hls_playlists: MediaHlsPlaylist[]
    
    created_at: string
    updated_at: string
}

export interface MediaObjects {
    kind: string
    storage_key: string
    content_type: string
    size: number
    name: string
    thumbhash: string
    created_at: string
    updated_at: string
    deleted_at: string
}

export interface MediaObjectMetadata {
    width: number
    height: number
    duration: number
    created_at: string
    updated_at: string
}

export interface MediaHls {
    master_playlist: string
    created_at: string
    updated_at: string
}

export interface MediaHlsPlaylist {
    resolution: string
    playlist_storage_key: string
    
    segment_count: number
    segment_duration: number

    created_at: string
    updated_at: string
}