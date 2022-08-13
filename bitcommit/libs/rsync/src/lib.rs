mod socket;


pub struct Stats {
    pub num_files: u64,
    /// Sum of the sizes of all the files in the source
    pub total_size: usize,
    /// Sum of the sizes of all the files that were synced
    pub total_transfered: u64,

    /// Number of files transfered (should match `num_files`
    /// if no error)
    pub num_synced: u64,
    /// Number of files for which the copy was skipped
    pub up_to_date: u64,
    /// Number of files that were copied
    pub copied: u64,
    /// Number of errors
    pub errors: u64,

    /// Number of symlink created in the destination folder
    pub symlink_created: u64,
    /// Number of symlinks updated in the destination folder
    pub symlink_updated: u64,

    /// Duration of the transfer
    pub duration: std::time::Duration,

}


pub struct Files {
    pub nr_files: u64,
    pub deph: u8,
}


pub struct Rsync {
    pub stats: Stats, 
    pub files: Files,
    pub authentivate: bool,
}



impl Rsync {


//  ----------         ----------
//  | server |         | client |
//  ----------         ----------
//
//  listen              trigger -> Struct
//    


    fn get_files(){
        
    }

}
