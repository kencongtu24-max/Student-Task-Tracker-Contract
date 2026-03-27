#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, String, Symbol};

// Khóa lưu tên công việc hiện tại
const TASK_KEY: Symbol = symbol_short!("TASK");

// Khóa lưu trạng thái hoàn thành
const STATUS_KEY: Symbol = symbol_short!("STATUS");

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Tạo hoặc cập nhật công việc mới.
    // Khi đặt task mới, trạng thái hoàn thành sẽ được reset về false.
    pub fn set_task(env: Env, task_name: String) {
        env.storage().instance().set(&TASK_KEY, &task_name);
        env.storage().instance().set(&STATUS_KEY, &false);
    }

    // Lấy tên công việc hiện tại.
    // Nếu chưa có task nào, trả về chuỗi mặc định.
    pub fn get_task(env: Env) -> String {
        env.storage()
            .instance()
            .get(&TASK_KEY)
            .unwrap_or(String::from_str(&env, "No task yet"))
    }

    // Đánh dấu công việc hiện tại là đã hoàn thành.
    pub fn complete_task(env: Env) {
        env.storage().instance().set(&STATUS_KEY, &true);
    }

    // Lấy trạng thái hoàn thành của công việc.
    // Nếu chưa có dữ liệu, trả về false.
    pub fn get_status(env: Env) -> bool {
        env.storage().instance().get(&STATUS_KEY).unwrap_or(false)
    }

    // Reset toàn bộ trạng thái task về mặc định.
    // Task sẽ trở thành "No task yet" và trạng thái là false.
    pub fn reset_task(env: Env) {
        env.storage()
            .instance()
            .set(&TASK_KEY, &String::from_str(&env, "No task yet"));
        env.storage().instance().set(&STATUS_KEY, &false);
    }
}
// Tạo task mới
stellar contract invoke \
--id CAX72JBBXQSZUOHBOJ44M2AYJ3YDITWHZTQSPY5O4QFLOPY7XSZAYLKL \
--source-account student \
--network testnet \
--send=yes \
-- set_task \
--task_name "Hoan thanh bai tap Stellar"

// Đọc task hiện tại
stellar contract invoke \
--id CAX72JBBXQSZUOHBOJ44M2AYJ3YDITWHZTQSPY5O4QFLOPY7XSZAYLKL \
--source-account student \
--network testnet \
--send=yes \
-- get_task \

// Đánh dấu hoàn thành
stellar contract invoke \
--id CAX72JBBXQSZUOHBOJ44M2AYJ3YDITWHZTQSPY5O4QFLOPY7XSZAYLKL \
--source-account student \
--network testnet \
--send=yes \
-- complete_task \

// Kiểm tra trạng thái
stellar contract invoke \
--id CAX72JBBXQSZUOHBOJ44M2AYJ3YDITWHZTQSPY5O4QFLOPY7XSZAYLKL \
--source-account student \
--network testnet \
--send=yes \
-- get_status \

// Reset task
stellar contract invoke \
--id CAX72JBBXQSZUOHBOJ44M2AYJ3YDITWHZTQSPY5O4QFLOPY7XSZAYLKL \
--source-account student \
--network testnet \
--send=yes \
-- reset_task \

// Kiểm tra lại task sau reset
stellar contract invoke \
--id CAX72JBBXQSZUOHBOJ44M2AYJ3YDITWHZTQSPY5O4QFLOPY7XSZAYLKL \
--source-account student \
--network testnet \
--send=yes \
-- get_task \
