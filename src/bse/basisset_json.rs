use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InputData {
    pub name: String,
    pub elements: HashMap<usize, InputElement>,
    pub description: String,
    pub revision_description: String,
    pub revision_date: String,
    pub version: String,
    pub function_types: Vec<String>,
    pub names: Vec<String>,
    pub tags: Vec<String>,
    pub family: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputElement {
    pub electron_shells: Vec<InputShell>,
    pub references: Vec<InputReferences>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputReferences {
    pub reference_description: String,
    pub reference_keys: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputShell {
    pub angular_momentum: Vec<usize>,
    pub exponents: Vec<String>,
    pub coefficients: Vec<Vec<String>>,
    pub function_type: String,
    pub region: String,
}

#[cfg(test)]
mod tests {
    use crate::bse::basisset_json::{InputData, InputElement, InputShell};
    use serde_json::from_str;

    #[test]
    fn test_sto2g() {
        let sto2g_string = r#"
        {
            "molssi_bse_schema": {"schema_type": "complete", "schema_version": "0.1"},
            "revision_description": "Data from Gaussian09",
            "revision_date": "2018-06-19",
            "elements": {
                "1": {
                    "electron_shells": [
                        {
                            "function_type": "gto",
                            "region": "",
                            "angular_momentum": [0],
                            "exponents": ["0.1309756377E+01","0.2331359749E+00"],
                            "coefficients": [["0.4301284983E+00","0.6789135305E+00"]]
                        }
                    ],
                    "references": [{
                            "reference_description": "STO-2G Minimal Basis (2 functions/AO)",
                            "reference_keys": ["hehre1969a"]
                        }
                    ]
                }
            },
            "version": "1",
            "function_types": ["gto"],
            "names": ["STO-2G"],
            "tags": [],
            "family": "sto",
            "description": "STO-2G Minimal Basis (2 functions/AO)",
            "role": "orbital",
            "auxiliaries": {},
            "name": "STO-2G"
        }"#;
        let data: InputData = from_str(&sto2g_string).unwrap();
        assert_eq!(data.version, 1.to_string());
        assert_eq!(data.family, "sto".to_string());
        assert_eq!(data.role, "orbital".to_string());
        assert_eq!(data.name, "STO-2G".to_string());
        assert_eq!(data.function_types, vec!["gto"]);
    }

    #[test]
    fn test_element() {
        let element_string = r#"
        {
        "references": [{    "reference_description": "STO-2G Minimal Basis (2 functions/AO)",
                            "reference_keys": ["hehre1969a"] }
                      ],
        "electron_shells": [
            {
              "function_type": "gto",
              "region": "",
              "angular_momentum": [0],
              "exponents": ["13.0107010"],
              "coefficients": [["0.19682158E-01"]]},
            {
              "function_type": "gto",
              "region": "",
              "angular_momentum": [0],
              "exponents": ["0.12194962"],
              "coefficients": [["1.0000000"]]}
        ]
        }
        "#
        .to_string();
        let data: InputElement = from_str(&element_string).unwrap();
        assert_eq!(
            data.references[0].reference_keys,
            vec!["hehre1969a".to_string()]
        );
        assert_eq!(
            data.references[0].reference_description,
            "STO-2G Minimal Basis (2 functions/AO)".to_string()
        );
        assert_eq!(data.electron_shells[0].function_type, "gto".to_string());
        assert_eq!(data.electron_shells[0].angular_momentum, vec![0]);
        assert_eq!(
            data.electron_shells[0].coefficients,
            vec![vec!["0.19682158E-01".to_string()]]
        );
    }

    #[test]
    fn test_shell() {
        let shell_string = r#"{
            "function_type": "gto",
            "region": "",
            "angular_momentum": [0],
            "exponents": ["13.0107010","1.9622572","0.44453796"],
            "coefficients": [["0.19682158E-01","0.13796524","0.47831935"]]
        }"#
        .to_string();

        let data: InputShell = from_str(&shell_string).unwrap();
        assert_eq!(data.angular_momentum[0], 0);
        assert_eq!(
            data.exponents,
            vec![
                "13.0107010".to_string(),
                "1.9622572".to_string(),
                "0.44453796".to_string()
            ]
        );
        assert_eq!(
            data.coefficients,
            vec![vec![
                "0.19682158E-01".to_string(),
                "0.13796524".to_string(),
                "0.47831935".to_string()
            ]]
        );
        assert_eq!(data.function_type, "gto".to_string());
        assert_eq!(data.region, "".to_string());
    }
}
