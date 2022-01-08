 use std::fs;
 use std::env;
 use hound;

/* initBanks()
{
    for folders in samples{
        addBank(); // Load wav files and prepare for playback
                  // Hold all of it in memory or do we have to load each bank when requested?
    }
    currentBank = 0;
}

loadNextBank(){
    load currentBank++;
} */

pub struct BankLoader {
    pub bank: Vec<Vec<(i16, i16)>>,
    bank_index: usize,
    max_bank_index: usize
}

impl BankLoader {
    pub fn new() -> BankLoader {
        let mut bl = BankLoader {
            bank: Vec::new(),
            bank_index: 0,
            max_bank_index: 0
        };
        let sample_path = env::current_dir().unwrap().join("samples");
        let mut paths : Vec<_> = fs::read_dir(sample_path).unwrap().collect();
        paths.sort_by_key(|path|{
            path.as_ref().unwrap().path()
        });

        let mut next_bank = 0;
        for p in paths{
            if p.as_ref().unwrap().path().to_str().unwrap() == format!("{:03}", next_bank)
            {
                if next_bank > 0
                {
                    bl.max_bank_index = bl.max_bank_index + 1;
                }
                next_bank = next_bank + 1;
            }
            else
            {
                println!("Couldn't open {:?}", p.unwrap().path());
            }
        }
        bl
    }

   fn load_bank(&mut self){
        self.bank = Vec::new();
        for k in 0..12 {
            let sample_path = format!("{}{:03}{}{:03}{}", "samples/" , self.bank_index , "/", k,".wav");
            let mut reader = hound::WavReader::open(sample_path.to_string()).expect(format!("Couldn't open {:?}", sample_path).as_str());
            let sample = reader.samples::<i16>();
            let mut converted_sample = Vec::new();
            let mut sample_index = 0;
            let mut channel_one_data = 0;
            for smp in sample{
                if smp.is_ok(){
                    let data = smp.unwrap();
                    if sample_index % 2 == 0
                    {
                        channel_one_data = data as i16;
                    }
                    else
                    {
                        converted_sample.push((channel_one_data, data as i16));
                    }
                    
                }
                sample_index = sample_index + 1;
            }
            self.bank.push(converted_sample);
        }
    }

    pub fn switch_to_next_bank(&mut self) -> (){
        if self.bank_index >= self.max_bank_index
        {
            self.bank_index = self.max_bank_index;
            return;
        }
        self.bank_index = self.bank_index + 1;
        self.load_bank();
    }

    pub fn switch_to_previous_bank(&mut self) -> (){
        if self.bank_index <= 0
        {
            self.bank_index = 0;
            return;
        }
        self.bank_index = self.bank_index - 1;
        self.load_bank();
    }
}