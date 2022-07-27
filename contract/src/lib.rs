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
    records: LookupMap<AccountId, Person>
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
    pub fn add_new_patitent(&mut self, patitent: Person) {
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
    pub fn add_contitions_to_patient(&mut self, conditions: Vec<Conditions>, pacient_addr: String) {
        let mut pacient: Person = self.records.get(&pacient_addr).unwrap();
        pacient.syndromes.extend(conditions);
        env::log(format!("New conditions added to patient, address: {}", pacient_addr).as_bytes());
        self.records.insert(&pacient_addr, &pacient);
    }

    //TODO: this is probably wrong
    pub fn convertToDoctor(&mut self, person_addr: String) {
        self.records.get(&person_addr).unwrap().is_doctor = true;
        
        // emit the event
        env::log(format!("Converted to doctor {}", person_addr).as_bytes());
    } 

    //Todo: this is also probably wrong
    pub fn removeDoctor(&mut self, person_addr: String) {
        self.records.get(&person_addr).unwrap().is_doctor = false;
        
        // emit the event
        env::log(format!("Removed doctor {}", person_addr).as_bytes());
    }


    pub fn addMedicalData(&mut self, medical_data: MedicalData, person_addr: String) {
        let mut pacient: Person = self.records.get(&person_addr).unwrap();
        pacient.medical_data.push(medical_data);
        env::log(format!("New medical data added to patient, address: {}", person_addr).as_bytes());
    }


    pub fn addDeparture(&mut self, person_addr: String) {
        self.records.get(&person_addr).unwrap().departed = true;
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
        let myPerson = self.records.get(&account).unwrap();
        let myMedicalData = myPerson.medical_data;

        //create a json and append it to the string
        let mut json = String::new();
        for medical_data in myMedicalData {
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
        let myPerson = self.records.get(&account).unwrap();
        let myConditions = myPerson.syndromes;
        
        return serde_json::to_string(&json!({
            "status": "OK", 
            "data": myConditions
        })).unwrap();
    }

    pub fn get_patient_data(&self, account: String) -> String {
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

/*     // `match` is similar to `switch` in other languages; here we use it to default to "Hello" if
    // self.records.get(&account_id) is not yet defined.
    // Learn more: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
    pub fn get_greeting(&self, account_id: String) -> String {
        match self.records.get(&account_id) {
            Some(greeting) => greeting,
            None => "Hello".to_string(),
        }
    } */
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
