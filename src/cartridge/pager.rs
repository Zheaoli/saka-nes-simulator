#[derive(Copy, Clone, Debug)]
pub enum PageSize {
    OneKB = 0x400,
    FourKB = 0x1000,
    EightKB = 0x2000,
    SixteenKB = 0x4000,
}

#[derive(Copy, Clone, Debug)]
pub enum Page {
    First(PageSize),
    Last(PageSize),
    Number(usize, PageSize),
    FromEnd(usize, PageSize),
}

pub struct Pager {
    pub data: Vec<u8>,
}

impl Pager {
    pub fn new(data: Vec<u8>) -> Self {
        Pager { data }
    }
    pub fn read(&self, page: Page, offset: u16) -> u8 {
        let i = self.index(page, offset);
        self.data[i]
    }
    pub fn write(&mut self, page: Page, offset: u16, value: u8) {
        let i = self.index(page, offset);
        self.data[i] = value;
    }
    fn page_count(&self, size: PageSize) -> usize {
        if self.data.len() % (size as usize) != 0 {
            panic!("Page size must divide evenly into data length")
        }

        self.data.len() / (size as usize)
    }
    fn index(&self, page: Page, offset: u16) -> usize {
        match page {
            Page::First(size) => self.index(Page::Number(0, size), offset),
            Page::Last(size) => {
                let last_page = self.page_count(size) - 1;
                self.index(Page::Number(last_page, size), offset)
            }
            Page::Number(n, size) => {
                let last_page = self.page_count(size) - 1;
                if (offset as usize) > (size as usize) {
                    panic!("Offset cannot exceed page bounds")
                }
                if n > last_page {
                    panic!("Page out of bounds")
                }
                n * (size as usize) + (offset as usize)
            }
            Page::FromEnd(n, size) => {
                let last_page = self.page_count(size) - 1;
                self.index(Page::Number(last_page - n, size), offset)
            }
        }
    }
}
