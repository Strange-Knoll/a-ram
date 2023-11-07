use std::error::Error;

#[derive(Clone)]
pub struct Token{
    data:[char;3]
}

impl Token{
    //converts a string into a vector of tokens
    //each token is 3 characters long
    pub fn tokenize(string:&str) -> Vec<Token>{
        let mut tokens = Vec::<Token>::new();
        let mut chars = string.chars();
        let mut token = Token{
            data:[' ';3]
        };
        let mut i = 0;
        while let Some(c) = chars.next(){
            token.data[i] = c;
            i += 1;
            if i == 3{
                tokens.push(token);
                token = Token{
                    data:[' ';3]
                };
                i = 0;
            }
        }
        tokens
    }
    //converts a vector of tokens into a string
    pub fn detokenize(tokens:Vec<Token>) -> String{
        let mut string = String::new();
        for token in tokens{
            for c in token.data.iter(){
                string.push(*c);
            }
        }
        string
    }
}

#[derive(Clone)]
pub struct StaticSector{
    pub tokens:usize,
    pub data:Vec<Token>,
}

impl StaticSector{
    pub fn new(data:&str) -> Self{
        let tokens = Token::tokenize(data);
        StaticSector{
            tokens:tokens.len(),
            data:tokens,
        }
    }
}

//a dynamic sector represents a rolling view of tokens
//where the tokens are added to the end and removed from the beginning
//in the order that data is allocated
#[derive(Clone)]
pub struct DynamicSector{
    pub tokens:usize,
    pub data:Vec<Token>,
    pub max_tokens:usize,
}

impl DynamicSector{
    pub fn new(max_tokens:usize) -> Self{
        DynamicSector{
            tokens:0,
            data:Vec::<Token>::new(),
            max_tokens:max_tokens,
        }
    }
}

#[derive(Clone)]
pub enum MemorySize{
    Normal,
    Large,
}

#[derive(Clone)]
pub struct Memory{
    allocated_tokens:usize,
    max_tokens:usize,
    static_sectors:Vec<StaticSector>,
    dynamic_sectors:Vec<DynamicSector>,
}

impl Memory{
    pub fn new(mem_size:MemorySize) -> Self{
        let max_tokens = match mem_size{
            MemorySize::Normal => 4096/2,
            MemorySize::Large => 16384/2,
        };
        Memory{
            allocated_tokens:0,
            max_tokens:max_tokens,
            static_sectors:Vec::<StaticSector>::new(),
            dynamic_sectors:Vec::<DynamicSector>::new(),
        }
    }

    pub fn add_static_sector(&mut self, data:&str) -> Result<usize, Box<dyn Error>>{
        let formatted_data = format!("{}\n\n", data);
        let sector = StaticSector::new(formatted_data.as_str());
        if sector.tokens + self.allocated_tokens > self.max_tokens{
            return Err("Memory full".into());
        }
        self.static_sectors.push(sector.clone());
        self.allocated_tokens += sector.clone().tokens;
        Ok(self.static_sectors.len() - 1)
    }

    pub fn add_dynamic_sector(&mut self, max_tokens:usize) -> Result<usize, Box<dyn Error>>{
        let sector = DynamicSector::new(max_tokens);
        if sector.tokens + self.allocated_tokens > self.max_tokens{
            return Err("Dynamic Sector Exceeds Memory Cap".into());
        }
        self.dynamic_sectors.push(sector.clone());
        self.allocated_tokens += max_tokens;
        Ok(self.dynamic_sectors.len() - 1)
    }

    //allocates a token to a dynamic sector
    pub fn alloc(&mut self, sector:usize, data:&str) -> Result<(), Box<dyn Error>>{
        
        let tokens = Token::tokenize(data);
        let max_tokens = self.dynamic_sectors[sector].max_tokens;
        let sector_data = &mut self.dynamic_sectors[sector].data;

        //for each token in tokens
        for (i, token) in tokens.iter().enumerate(){
            //push the token to the end of the vec
            sector_data.push(token.clone());
            //if our vec is larger than max tokens
            if sector_data.len() > max_tokens{
                //remove the first element of the vec
                sector_data.remove(0);
            }
        }

        Ok(())
    }

    pub fn get_static_sector(&self, sector:usize) -> Result<&StaticSector, Box<dyn Error>>{
        if sector >= self.static_sectors.len(){
            return Err("Static Sector Index Out of Bounds".into());
        }
        Ok(&self.static_sectors[sector])
    }

    pub fn get_dynamic_sector(&self, sector:usize) -> Result<&DynamicSector, Box<dyn Error>>{
        if sector >= self.dynamic_sectors.len(){
            return Err("Dynamic Sector Index Out of Bounds".into());
        }
        Ok(&self.dynamic_sectors[sector])
    }
    

    pub fn to_string(&self) -> String{
        let mut string = String::new();
        for sector in self.static_sectors.iter(){
            string.push_str(Token::detokenize(sector.data.clone()).as_str());
        }
        for sector in self.dynamic_sectors.iter(){
            string.push_str(Token::detokenize(sector.data.clone()).as_str());
        }
        string
    }
}