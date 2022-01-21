use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct InputMetaData {
    pub(crate) basename: String,
    description: String,
    display_name: String,
    family: String,
    function_types: Vec<String>,
    latest_version: String,
    notes_exist: Vec<bool>,
    other_names: Vec<String>,
    relpath: String,
    role: String,
    tags: Vec<String>,
    versions: BTreeMap<String, InputVersions>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputVersions {
    elements: Vec<String>,
    file_relpath: String,
    revdate: String,
    revdesc: String,
}

#[cfg(test)]
mod tests {
    use crate::bse::metadata_json::InputMetaData;
    use serde_json::from_str;

    #[test]
    fn test_metadata() {
        let metadata_string = r#"
    {
      "auxiliaries":{},
      "basename":"STO-2G",
      "description":"STO-2G Minimal Basis (2 functions/AO)",
      "display_name":"STO-2G",
      "family":"sto",
      "function_types":["gto","gto_spherical"],
      "latest_version":"1",
      "notes_exist":[true, true],
      "other_names":[],
      "relpath":"",
      "role":"orbital",
      "tags":[],
      "versions":{
         "0":{
            "elements":["1","2","3","4","5","6","7","8","9","10","11","12","13","14","15","16",
                        "17","18","19","20","38"],
            "file_relpath":"STO-2G.0.table.json",
            "revdate":"January 15, 2007",
            "revdesc":"Data from the Original Basis Set Exchange"
         },
         "1":{
            "elements":["1","2","3","4","5","6","7","8","9","10","11","12","13","14","15","16",
                        "17","18","19","20","21","22","23","24","25","26","27","28","29","30",
                        "31","32","33","34","35","36","37","38","39","40","41","42","43","44",
                        "45","46","47","48","49","50","51","52","53","54"],
            "file_relpath":"STO-2G.1.table.json",
            "revdate":"June 19, 2018",
            "revdesc":"Data from Gaussian09"
         }
      }
   }"#;
        let data: InputMetaData = from_str(&metadata_string).unwrap();
        assert_eq!(&data.basename, "STO-2G");
        assert_eq!(&data.description, "STO-2G Minimal Basis (2 functions/AO)");
        assert_eq!(&data.display_name, "STO-2G");
        assert_eq!(data.notes_exist, vec![true, true]);
    }

}
