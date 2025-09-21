# Advanced Rust Payroll System API

This project is a high-performance, next-generation payroll management system built with Rust, Axum, and PostgreSQL. It's designed to be a robust backend supporting cutting-edge features like AI-powered analytics, real-time payroll processing, blockchain integration, and advanced employee self-service.

## Features
- **High Performance:** Built with Rust and the Axum framework for speed and safety.
- **Asynchronous:** Fully async from the web layer down to the database.
- **Modern Database:** Uses PostgreSQL with `sqlx` for compile-time query checking.
- **Advanced Data Models:** Supports complex JSON-based data structures for analytics, benefits, and more.
- **Ready for the Future:** Designed to integrate AI, blockchain, and real-time payment features.

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [PostgreSQL](https://www.postgresql.org/download/)
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (`cargo install sqlx-cli`)

### Installation & Running
1.  **Clone the repository:**
    ```bash
    git clone <your-repo-url>
    cd advanced-payroll
    ```

2.  **Set up the environment:**
    Create a `.env` file in the project root and add your database URL:
    ```.env
    DATABASE_URL="postgres://user:password@localhost/your_db_name"
    ```

3.  **Run database migrations:**
    This will create and update your database tables to match the application's models.
    ```bash
    sqlx migrate run
    ```

4.  **Run the application:**
    ```bash
    cargo run --release
    ```
    The server will start on `http://0.0.0.0:3000`.

---

## 🚀 API Documentation

This API provides endpoints for managing employees and advanced payroll operations. For a live, interactive experience, open the `api-docs/scalar.html` or `api-docs/swagger.html` files in your browser.

### Employee Management

---

#### **`POST /employees`**
Creates a new employee record in the system.

**Request Body** (`application/json`)
```json
{
  "first_name": "Jane",
  "last_name": "Doe",
  "email": "jane.doe@example.com",
  "department": "Engineering",
  "position": "Senior Software Developer",
  "salary_info": {
    "base_salary": 120000,
    "currency": "USD",
    "pay_frequency": "BiWeekly",
    "overtime_rate": 55.0
  }
}
```

**Success Response (200 OK)** (`application/json`)
```json
{
  "id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
  "employee_id": "EMP-12345",
  "first_name": "Jane",
  "last_name": "Doe",
  "email": "jane.doe@example.com",
  "phone": null,
  "hire_date": "2025-09-21",
  "department": "Engineering",
  "position": "Senior Software Developer",
  "employment_type": "FullTime",
  "status": "Active",
  "salary_info": {
    "base_salary": 120000,
    "currency": "USD",
    "pay_frequency": "BiWeekly",
    "overtime_rate": 55.0
  },
  "benefits": [],
  "tax_info": {},
  "banking_info": null,
  "created_at": "2025-09-21T18:00:00Z",
  "updated_at": "2025-09-21T18:00:00Z"
}
```

---

#### **`GET /employees`**
Retrieves a list of all employees, ordered by the most recently created.

**Success Response (200 OK)** (`application/json`)
```
[
  {
    "id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "employee_id": "EMP-12345",
    "first_name": "Jane",
    "last_name": "Doe",
    "email": "jane.doe@example.com",
    // ... other fields
  }
]
```

---

#### **`GET /employees/{id}`**
Retrieves a single employee by their unique UUID.

**Path Parameters**
- `id` (string, required): The UUID of the employee.

**Success Response (200 OK)** (`application/json`)
*Returns the full employee object as shown in the `POST /employees` example.*

**Error Response (404 Not Found)**
*Returned if no employee with the given ID exists.*

---

### Advanced Payroll Features (Placeholders)

---

#### **`POST /payroll/run`**
Initiates a new payroll run. This is an asynchronous operation.

**Success Response (200 OK)** (`application/json`)
```json
{
  "status": "Payroll run initiated successfully",
  "transaction_id": "f4g5h6j7-k8l9-1011-1213-141516abcdef"
}
```

---

#### **`GET /analytics/reports`**
Fetches AI-powered analytics and insights on payroll data.

**Success Response (200 OK)** (`application/json`)
```json
{
  "fraud_score": 0.95,
  "compliance_score": 0.99,
  "insights": [
    "Anomaly detected: 15% increase in overtime for the engineering department.",
    "Prediction: Total payroll cost expected to rise by 2% next quarter."
  ]
}
```

---

#### **`POST /chatbot/query`**
Sends a query to the AI-powered payroll assistant chatbot.

**Success Response (200 OK)** (`application/json`)
```
{
  "response": "Your current net pay for this period is $2,543.12."
}
```
---

#### **`GET /employee/{id}/dashboard`**
Retrieves the self-service dashboard information for a specific employee.

**Path Parameters**
- `id` (string, required): The UUID of the employee.

**Success Response (200 OK)** (`application/json`)
```json
{
  "employee_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
  "available_earned_wages": 450.75,
  "ytd_gross_pay": 55123.4
}
```