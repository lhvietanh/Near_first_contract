//! Contract này thực hiện counter đơn giản được hỗ trợ bởi lưu trữ trên blockchain.
//!
//! Contract cung cấp các method để [increment] / [decrement] counter và
//! [lấy value hiện tại của nó][get_num] hoặc [reset].
//!
//! [increment]: struct.Counter.html#method.increment
//! [decrement]: struct.Counter.html#method.decrement
//! [get_num]: struct.Counter.html#method.get_num
//! [reset]: struct.Counter.html#method.reset

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ env,  near_bindgen, };
near_sdk::setup_alloc!();


// thêm các thuộc tính sau để chuẩn bị code của bạn cho serialization và gọi trên blockchain
// Các thuộc tính Rust được tích hợp sẵn khác tại đây: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    // Xem thêm các loại dữ liệu tại https://doc.rust-lang.org/book/ch03-02-data-types.html
    val: i8, // i8 được sign. các unsign integer sẵn có là: u8, u16, u32, u64, u128
    account_id: String,
    //list_ticket: u16,
    num_ticket: u8,
}


#[near_bindgen]
impl Counter {

    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        let acc_id = env::current_account_id();
        Counter {
            val :0,
            
            account_id: acc_id.to_string() ,
            num_ticket: 0,
        }
    }


    /// Trả về sign integer 8 bit của giá trị counter.
    ///
    /// This must match the type from our struct's 'val' defined above.
    ///
    /// Note, the parameter is `&self` (without being mutable) meaning it doesn't modify state.
    /// In the frontend (/src/main.js) this is added to the "viewMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near view counter.YOU.testnet get_num
    /// ```

    pub fn get_num(&self) -> i8 {
        return self.val;
    }
    pub fn get_acc(&self) -> String{
        return self.account_id.to_string();
    }
    pub fn get_um_ticket(&self) -> u8{
        return self.num_ticket;
    }
    pub fn set_num_ticket(&mut self,num_ticket: u8) {
        self.num_ticket =  num_ticket;
    }
    /// Increment the counter.
    ///
    /// Note, the parameter is "&mut self" as this function modifies state.
    /// In the frontend (/src/main.js) this is added to the "changeMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near call counter.YOU.testnet increment --accountId donation.YOU.testnet
    /// ```
    pub fn increment(&mut self) {
        // note: adding one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        // e.g. self.val = i8::wrapping_add(self.val, 1);
        // https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_add
        self.val += 1;
        let log_message = format!("Increased number to {}", self.val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    /// Decrement (subtract from) the counter.
    ///
    /// In (/src/main.js) this is also added to the "changeMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near call counter.YOU.testnet decrement --accountId donation.YOU.testnet
    /// ```
    pub fn decrement(&mut self) {
        // note: subtracting one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        // e.g. self.val = i8::wrapping_sub(self.val, 1);
        // https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_sub
        self.val -= 1;
        let log_message = format!("Decreased number to {}", self.val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    /// Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        // Một cách khác để log là truyền một chuỗi thành các byte, như "b" bên dưới:
        env::log(b"Reset counter to zero");
    }
}

// không giống như các function của struct ở trên, function này không thể sử dụng các thuộc tính #[derive(…)] hoặc #[near_bindgen]
// bất kỳ nỗ lực nào cũng sẽ đưa ra những cảnh báo hữu ích khi 'cargo build'
// trong khi function này không thể được gọi trực tiếp trên blockchain, nó có thể được gọi từ một function được gọi
fn after_counter_change() {
    // show helpful warning that i8 (8-bit signed integer) will overflow above 127 or below -128
    // hiển thị cảnh báo hữu ích rằng i8 (số nguyên có dấu 8 bit) sẽ tràn trên 127 hoặc thấp hơn -128
    env::log("Make sure you don't overflow, my friend.".as_bytes());
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-counter-tutorial -- --nocapture
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// sử dụng các thuộc tính bên dưới cho các unit test
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // một phần của viết các unit test là thiết lập một mock context
    // trong ví dụ này, điều này chỉ cần thiết cho env::login trong contract
    // đây cũng là một danh sách hữu ích để xem khi tự hỏi những gì có sẵn trong env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // đánh dấu các unit test riêng lẻ bằng #[test] để chúng được đăng ký và bắt đầu
    #[test]
    fn increment() {
        // thiết lập ngữ cảnh mock vào môi trường test
        let context = get_context(vec![], false);
        testing_env!(context);
        // khởi tạo một biến contract với counter bằng 0
        
       // let acc_id = context.signer_account_id.;

        let mut contract = Counter::new();
        contract.set_num_ticket(20);
        contract.increment();
        println!("Value after increment: {}", contract.get_num());
        println!("current_account_id:  {}",contract.get_acc());
        println!("total ticket  {}",contract.get_um_ticket());

        // xác nhận rằng chúng ta đã nhận được 1 khi gọi get_num
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Counter::new();
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num());
        // xác nhận rằng chúng ta đã bị -1 khi gọi get_num
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Counter::new();
        contract.increment();
        contract.reset();
        println!("Value after reset: {}", contract.get_num());
        // xác nhận rằng chúng ta đã được -1 khi gọi get_num
        assert_eq!(0, contract.get_num());
    }
}