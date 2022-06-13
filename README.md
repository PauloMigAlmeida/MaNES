# MaNES

MaNES is an experimental NES emulator written from scratch in Rust. This is work in progress and I would expect something to have it "finalised" by the end of 2022.

## What MaNES stands for?
I'm paying homage to my grandfather whose name is Manoel so "Mane" is the short version of it. In some contexts "Mane" is used as a derogatory term to indicate that someone is stupid. 

My grandfather has always been my role model and extremely versatile. There is not a thing that he wasn't able to fix/amend due to his restless curiosity which is even more impressive when you take into account that his was pretty much illiterate. 

Throughout his life, he managed to turn the word "Mane" into some sort of seal of quality for whatever he worked on. For instance, he would fix the car and proudly refer to himself as "Mane-Mechanic" with a big smile on his face.

He is pretty old now (83ish) and I would like to show him the "Mane Nintendo Entertainment System" to see him smile again with that joke :)

[Update: 05/06/2022] :: the doctor said his heart is very frail and that he could pass away at anytime now. Unfortunately, I don't think I will have enough time to finish this project before the worst happens... so I took the time to explain to him what this is and why I named it after him... I got the laugh I was looking for. That will live in my memory forever. 

![7724670e-c379-4557-9f53-9c2411c6f19e](https://user-images.githubusercontent.com/1011868/173266995-fe94c1d2-42fa-4922-9877-95becbaa71a6.jpg)
![f7dbb2fc-d2e2-4557-aa73-28351546c692](https://user-images.githubusercontent.com/1011868/173267217-6a0f3375-acab-41fa-be86-a5b55522f0cf.jpg)

## Wishlist
To make sure I won't lose focus on what I want this emulator to be able to do, I decided to write a list of features
that I want to implement in the short to medium term.

- [X] Emulate legal opcodes for the 6502
- [X] Create a ROM Disassembler (somewhat similar to what objdump does)
- [ ] Create simplified GUI that contains the framebuffer and a RAM view
- [ ] Implement 1 Mapper (the simplest one) 
- [ ] Write all above in Rust (that's the secondary goal for this winter project)


## Stretch goals
Things that would be fantastic to have but I am not sure that I want to spend all that time

- [ ] Implement most commons Mappers
- [ ] Audio emulation

## References
These are all the references that helped me a lot during the development of AlmeidaOS

Forums:
- https://www.nesdev.org/wiki/Nesdev_Wiki

Books:
- https://www.amazon.com/Rust-Rustaceans-Programming-Experienced-Developers/dp/1718501854/ 

## Dependencies

For Mac OS:
```{bash}
brew install gtk4
```

For Fedora:
```{bash}
sudo dnf install gtk4-devel
```
