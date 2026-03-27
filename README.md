# Student-Task-Tracker-Contract
# Title
Student Task Tracker Contract

# Description
Đây là một dự án smart contract được xây dựng trên nền tảng Stellar Soroban nhằm hỗ trợ người dùng quản lý một công việc học tập hoặc nhiệm vụ cá nhân đơn giản trên blockchain.

Dự án cho phép người dùng tạo một task, xem lại task hiện tại, đánh dấu task đã hoàn thành và reset lại trạng thái khi muốn bắt đầu một nhiệm vụ mới.

Mục đích của dự án là thực hành quy trình phát triển smart contract trên Stellar, bao gồm:
- viết contract bằng Rust
- build contract sang WASM
- deploy contract lên Stellar testnet
- tương tác với contract bằng Stellar CLI

Ý tưởng này được chọn vì mang tính thực tế hơn so với ví dụ lưu câu chào đơn giản. Việc quản lý task là nhu cầu gần gũi với sinh viên, người học hoặc người làm việc cá nhân. Dự án vừa đủ đơn giản để dễ triển khai, nhưng vẫn thể hiện rõ cách lưu trữ và cập nhật dữ liệu trên blockchain.

# Tính năng
- Tạo hoặc cập nhật một công việc mới
- Xem công việc hiện tại đang được lưu
- Đánh dấu công việc là đã hoàn thành
- Kiểm tra trạng thái hoàn thành của công việc
- Reset dữ liệu để bắt đầu lại từ đầu
- Tương tác trực tiếp với smart contract trên Stellar testnet

# Contract
Link contract:  
https://stellar.expert/explorer/testnet/contract/CAX72JBBXQSZUOHBOJ44M2AYJ3YDITWHZTQSPY5O4QFLOPY7XSZAYLKL

Ảnh chụp màn hình contract:  
<img width="1024" height="666" alt="Ảnh màn hình 2026-03-27 lúc 22 35 26" src="https://github.com/user-attachments/assets/8144be18-e652-45a0-b194-4877e29e8578" />

# Future scopes
Trong tương lai, dự án có thể được mở rộng theo nhiều hướng thực tế hơn như:

- Quản lý nhiều task thay vì chỉ một task duy nhất
- Lưu danh sách task theo từng người dùng hoặc từng địa chỉ ví
- Thêm thời gian tạo task và thời hạn hoàn thành
- Phân loại task theo mức độ ưu tiên
- Xây dựng giao diện frontend để người dùng thao tác trực quan
- Kết nối ví Stellar để mỗi người dùng quản lý dữ liệu riêng
- Phát triển thành một ứng dụng quản lý học tập phi tập trung trên Web3

Về lâu dài, mình muốn phát triển ý tưởng này thành một công cụ hỗ trợ học tập cá nhân trên blockchain, nơi người dùng có thể theo dõi tiến độ công việc, lưu lịch sử hoàn thành task và tạo động lực học tập thông qua hệ thống minh bạch, không thể chỉnh sửa tùy ý.

# Profile
- Họ và tên: [Võ Bá Quốc Đạt]
- Vai trò: Sinh viên / Người học blockchain
- Kỹ năng: Rust cơ bản, Stellar Soroban, Stellar CLI, smart contract cơ bản
- Mục tiêu: Học cách xây dựng, deploy và tương tác với smart contract trên Stellar
- Sở thích: Blockchain, Web3, lập trình smart contract, công nghệ phi tập trung
