# A-Ram
NOTE: this api is deprecated thanks to the introduction of Threads in the openai api on Nov 6, 2023. This project is now as a proof of concept.
### AI Memory
a-ram is a framework designed to handle storing memory data for gpt-3.5 systems and assistants. a-ram contains two memory sector types Static Sectors and Dynamic Sectors

### Memory
a-ram supports two memory sizes Normal and Large. Normal memory is limited to 2048 tokens, large memory is limited to 8192 tokens. This is because we are limited to 2 token maximums in gpt 3.5 depending on the model, 4097 and 16385. this is divided between the users query, system, and assistant and the response. Since we cannot know for certain the size of the response ahead of time, we assume a worst case scenario that the response will be fully half of the memory limit. The sum total of allocated tokens added to memory cannot exceed these memory limits. memory is divided onto two sections, Dynamic Sectors and Static Sectors. The size limit available to dynamic sectors is dependent on the size of the static sectors in memory.

### Tokens
Tokens as implemented are only a approximation of the actual tokens gpt processes 

### Static Sector
a static sector is declared once and is immutable. once it is declared the memory allocated is subtracted from the total store of memory left for other sectors.
static sectors are lossless and will always be present in memory. static sectors are allocated as follows:
```
let mut memory = Memory::new(MemorySize::Large);
let stat_sect = memory.add_static_sector("you are a chat bot.")?;
```
add_static_sector return the index of the static sector in memory. this sector can then be retrieved from memory as follows:
```
let my_static_sector = memory.get_static_sector(stat_sect)?;
```


### Dynamic Sector
a dynamic sector of memory is mutable however the size limit of the dynamic sector is allocated upon instantiation. dynamic memory represents a rolling view of tokens where the most recent tokens are added to the end of the sector, and tokens at the beginning of the sector are removed as the sector reaches its defined memory limit. This means Dynamic memory is fundamentally lossy. data is regularly removed from the top of the array to make room for more recent data at the end of the array. This means that older memory (such as the beginnings of chat logs) will be forgotten. dynamic memory is instantiated as follows:
```
let mut memory = Memory::new(MemorySize::Large);
let dyn_sect = memory.add_dynamic_sector(128)?;
```
add_dynamic_sector returns the index of the dynamic sector in memory
data is allocated as follows:
```
memory.alloc(dyn_sect, "your data here")?;
memory.alloc(dyn_sect, "more data here")?;
memory.alloc(dyn_sect, "even more data here")?;
```
and can be retrieved from memory as follows:
```
let my_dynamic_sector = memory.get_dynamic_sector(dyn_sect)
```
this returned sector which, when we call ```.to_string()``` on it, returns:
```
your data here
more data here
even more data here
```
### Things to know
memory sectors are designed to be read from the memory into a System, Query, or Assistant as defined in the openai documentation. Memory is able to allocate space under the assumption that a sector is only written Once to One of those systems. if you were to append the same sector to both the System and the Assistant, you will loose the safety limits provided by memory and could over run token limits. 

you will need an open-ai api key to run main.rs. copy and paste your key into the KEY variable in main.rs