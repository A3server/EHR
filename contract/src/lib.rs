/// Import `borsh` from `near_sdk` crate 
// Import `serde` from `serde_json` crate
use near_sdk::{
    env,
    setup_alloc,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{ LookupMap },
    serde::{Serialize, Deserialize},
    AccountId,
    serde_json::{self, json},
    near_bindgen,
};


setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    records: LookupMap<AccountId, Person>,
    admin_accounts: Vec<AccountId>
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Conditions {
    name_of_syndrome: String,
    date: i32
}
impl Conditions {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
} 

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Medicine {
    active_compound: String,
    dose: i8,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Person {
    person_addr: String,
    nr_utente_saude: String,
    name: String,
    surname: String,
    placeof_birth: String,
    family_history: String,
    birth_date: i32,
    is_doctor: bool,
    departed: bool,
    medical_data: Vec<MedicalData>,
    syndromes: Vec<Conditions>,
}

impl Person {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn is_none(&self) -> bool {
        self.person_addr.is_empty()
    }
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MedicalData {
    doctor: String,
    notes: String,
    tobbaco_load: String,
    treatment: String,
    medicine_taken: Vec<Medicine>,
    date: i32,
}

impl MedicalData {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}


impl Default for Contract {
  fn default() -> Self {
    //TODO: add edup.testnet as a default
    Self {
      records:  LookupMap::new(b"s".to_vec()),
        admin_accounts: vec![env::signer_account_id()]
    }

  }
}

#[near_bindgen]
impl Contract {
    /* New patitent comes in:
    {
        "person_addr": "polpy.testnet",
        "nr_utente_saude": "123456789",
        "name": "John",
        "surname": "Doe",
        "placeof_birth": "Porto",
        "family_history": "mom with problems",
        "birth_date": 123456789,
        "is_doctor": false,
        "departed": false,
        "medical_data": [
            {
                "doctor": "doctor.testnet",
                "notes": "None",
                "tobbaco_load": "None",
                "treatment": "None",
                "medicine_taken": [
                    {
                        "active_compound": "None",
                        "dose": 0
                    }
                ],
                "date": 123456789
            }
        ],
        "syndromes": [
            {
                "name_of_syndrome": "None",
                "date": 123456789
            }
        ]
    }
    '{"person_addr":"polpy.testnet","nr_utente_saude":"123456789","name":"John","surname":"Doe","placeof_birth":"Porto","family_history":"mom with problems","birth_date":123456789,"is_doctor":false,"departed":false,"medical_data":[{"doctor":"doctor.testnet","notes":"None","tobbaco_load":"None","treatment":"None","medicine_taken":[{"active_compound":"None","dose":0}],"date":123456789}],"syndromes":[{"name_of_syndrome":"None","date":123456789}]}'
    near call ehr.polpy.testnet add_new_patitent '{"patitent":{"person_addr":"polpy.testnet","nr_utente_saude":"123456789","name":"John","surname":"Doe","placeof_birth":"Porto","family_history":"mom with problems","birth_date":123456789,"is_doctor":false,"departed":false,"medical_data":[{"doctor":"doctor.testnet","notes":"None","tobbaco_load":"None","treatment":"None","medicine_taken":[{"active_compound":"None","dose":0}],"date":123456789}],"syndromes":[{"name_of_syndrome":"None","date":123456789}]}}' --accountId polpy.testnet

    */
    pub fn add_new_patitent(&mut self, patitent: Person)  {
        self.records.insert(&patitent.person_addr, &patitent);

        env::log(b"Patient added");
    }

    /*
    conditions come in
    {
        "conditions": [
            {
                "name_of_syndrome": "None2",
                "date": 123456789
            },
            {
                "name_of_syndrome": "None3",
                "date": 123456789
            }
        ],
        "pacient_addr": "polpy.testnet"
    }
    near call ehr.polpy.testnet add_contitions_to_patient '{"conditions":[{"name_of_syndrome":"None2","date":123456789},{"name_of_syndrome":"None3","date":123456789}],"pacient_addr":"polpy.testnet"}' --accountId polpy.testnet
    */
    pub fn add_contitions_to_patient(&mut self, conditions: Vec<Conditions>, pacient_addr: String) -> bool {
        // Check if current accountId callee is a doctor
        if !self.is_doctor(env::signer_account_id()) {
            env::log(b"You are not a doctor");
            return false;
        }


        let mut pacient: Person = self.records.get(&pacient_addr).unwrap();
        assert!(!pacient.is_none(), "Pacient is none");


        pacient.syndromes.extend(conditions);
        self.records.insert(&pacient_addr, &pacient);
        env::log(format!("New conditions added to patient, address: {}", pacient_addr).as_bytes());
        true
    }

    pub fn convert_to_doctor(&mut self, account: String, value: bool) -> bool {
        let mut pacient: Person = self.records.get(&account).unwrap();
        assert!(!pacient.is_none(), "Pacient is none");

        if value == pacient.is_doctor {
            env::log(format!("Unchanged").as_bytes());
            return false;
        }


        pacient.is_doctor = value;
        self.records.insert(&account, &pacient);
        env::log(format!("Doctor status from account {} changed to: {}",account, value).as_bytes());
        true
    } 



    pub fn add_medical_data(&mut self, medical_data: MedicalData, person_addr: String) {
        let mut pacient: Person = self.records.get(&person_addr).unwrap();
        assert!(!pacient.is_none(), "Pacient is none");


        pacient.medical_data.push(medical_data);
        self.records.insert(&person_addr, &pacient);
        
        env::log(format!("New medical data added to patient, address: {}", person_addr).as_bytes());
    }

    pub fn remove_medical_data(&mut self, person_addr: String, index: i32) {
        let mut pacient: Person = self.records.get(&person_addr).unwrap();
        assert!(!pacient.is_none(), "Pacient is none");

        pacient.medical_data.remove(index as usize);
        self.records.insert(&person_addr, &pacient);
        
        env::log(format!("Medical data removed from patient, address: {}", person_addr).as_bytes());
    }

    // :(
    pub fn add_departure(&mut self, person_addr: String) {
        let mut pacient: Person = self.records.get(&person_addr).unwrap();
        assert!(!pacient.is_none(), "Pacient is none");

        pacient.departed = true;
        self.records.insert(&person_addr, &pacient);
        env::log(format!("Departure added to patient, address: {}", person_addr).as_bytes());
    }




    /*
     * GETTERS
     */
    pub fn get_medical_data(&self, account: String) -> String {
        //i literally thought that we could get the signer_id here to get the medical data from the person who is calling the method but then i realized that we can't get the signer_id since this is a view method :)
        if self.records.get(&account).is_none() {
            let string = format!("No patitent found with the address: {}", account);
            return serde_json::to_string(&json!({
                "status": string, 
                "data": []
            })).unwrap();
        }
        let my_person = self.records.get(&account).unwrap();
        let my_med_data = my_person.medical_data;

        //create a json and append it to the string
        let mut json = String::new();
        for medical_data in my_med_data {
            json.push_str(&medical_data.to_json());
        }
        json
    }

    pub fn get_conditions(&self, account: String) -> String {
        if self.records.get(&account).is_none() {
            let string = format!("No patitent found with the address: {}", account);
            return serde_json::to_string(&json!({
                "status": string, 
                "data": []
            })).unwrap();
        }
        let my_person = self.records.get(&account).unwrap();
        let my_conditions = my_person.syndromes;
        
        return serde_json::to_string(&json!({
            "status": "OK", 
            "data": my_conditions
        })).unwrap();
    }

    pub fn get_patient(&self, account: String) -> String {
        if self.records.get(&account).is_none() {
            let string = format!("No patitent found with the address: {}", account);
            return serde_json::to_string(&json!({
                "status": string, 
                "data": []
            })).unwrap();
        }
        let my_data = self.records.get(&account).unwrap();
        my_data.to_json()
    }

    fn is_doctor(&self, account: String) -> bool {
        if self.records.get(&account).is_none() {
            return false;
        }
        let my_data = self.records.get(&account).unwrap();
        my_data.is_doctor
    }
}




/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    
}
