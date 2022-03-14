pub mod postgresql {
    use postgres::{Client, Error, NoTls};
    use skytable::actions::Actions;
    use skytable::sync::Connection;

    /// The structure of the generator failure.
    pub struct Faulty {
        generator_faulty: i32,
    }

    /// The structure of a UNIX timestamp with the time zone of the last value entry in the table.
    pub struct UnixFromSql {
        time: f64,
    }

    /// The structure of a UNIX timestamp with the time zone now.
    pub struct UnixFromSqlNow {
        time: f64,
    }

    /// The structure of the signal of the presence of the opc server connection with the plc.
    pub struct PlcConnect {
        connection: i32,
    }

    pub fn db_connect() -> String {
        let string_connection =
            String::from("postgresql://stepanov:postgres@localhost:5433/postgres");
        string_connection
    }

    /// Set default transaction isolation level for database
    pub fn set_transaction_isolation() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "alter database postgres set default_transaction_isolation to serializable",
        )?;
        Ok(())
    }

    /// Create SQL table "avr_control_insert"
    pub fn create_avr_control_insert_table() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "
                CREATE TABLE IF NOT EXISTS avr_control_insert (
                    mains_power_supply int NOT NULL,
                    start_generator int NOT NULL,
                    generator_faulty int NOT NULL,
                    generator_work int NOT NULL,
                    connection int NOT NULL,
                    mark timestamptz default current_timestamp
                )
            ",
        )?;
        Ok(())
    }

    /// Create SQL table "журнал_работы_приложения"
    pub fn create_log_of_work_app_table() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "
                CREATE TABLE IF NOT EXISTS журнал_работы_приложения (
                    событие text NOT NULL,
                    время_и_дата timestamp default current_timestamp
                )
            ",
        )?;
        Ok(())
    }

    /// Create SQL table "зимний_сад"
    pub fn create_winter_garden_table() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "
                CREATE TABLE IF NOT EXISTS зимний_сад (
                    фитоосвещение_1 int NOT NULL,
                    фитоосвещение_2 int NOT NULL,
                    фитоосвещение_3 int NOT NULL,
                    фитоосвещение_4 int NOT NULL,
                    вентилятор int NOT NULL,
                    автополив_1 int NOT NULL,
                    автополив_2 int NOT NULL,
                    автополив_3 int NOT NULL,
                    температура int NOT NULL,
                    влажность int NOT NULL,
                    освещенность_в_помещении int NOT NULL,
                    освещенность_на_улице int NOT NULL,
                    время_и_дата timestamp default current_timestamp
                )
            ",
        )?;
        Ok(())
    }

    /// Create SQL table "нагрузка_на_генератор"
    pub fn create_generator_load_table() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "
                CREATE TABLE IF NOT EXISTS нагрузка_на_генератор (
                    нагрузка int NOT NULL,
                    время_и_дата timestamp default current_timestamp
                
                )
            ",
        )?;
        Ok(())
    }

    /// Create SQL table "события_авр"
    pub fn create_avr_events_table() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "
                CREATE TABLE IF NOT EXISTS события_авр (
                    событие text NOT NULL,
                    время_и_дата timestamp default current_timestamp
                )
            ",
        )?;
        Ok(())
    }

    /// Records the event "Авария! Генератор неисправен! Срочно произведите сервисные работы!" in the sql table "события_авр".
    pub fn event_generator_work_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Авария! Генератор неисправен! Срочно произведите сервисные работы!";
        client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Работоспособность генератора восстановлена. Генератор исправен. Генератор работает." in the sql table "события_авр".
    pub fn event_generator_work_restored() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event =
            "Работоспособность генератора восстановлена. Генератор исправен. Генератор работает.";
        client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Генератор в режиме трансляции питания от электросети работает исправно." in the sql table "события_авр".
    pub fn event_generator_work_ok() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Генератор в режиме трансляции питания от электросети работает исправно.";
        client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records log "Ошибка! Связь ПЛК с модулем modbus_ats отсутствует!" in the sql table "журнал_работы_приложения".
    pub fn log_timeout_or_host_unreachable_modbus_ats() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Ошибка! Связь ПЛК с модулем modbus_ats отсутствует!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Ошибка! Связь ПЛК с модулем modbus_winter_garden отсутствует!" in the sql table "журнал_работы_приложения".
    pub fn log_timeout_or_host_unreachable_modbus_winter_garden() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Ошибка! Связь ПЛК с модулем modbus_winter_garden отсутствует!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Авария! Генератор неисправен! Срочно произведите сервисные работы!" in the sql table "журнал_работы_приложения".
    pub fn log_generator_work_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Авария! Генератор неисправен! Срочно произведите сервисные работы!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Генератор в режиме трансляции питания от электросети работает исправно." in the sql table "журнал_работы_приложения".
    pub fn log_generator_work_ok() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Генератор в режиме трансляции питания от электросети работает исправно.";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Работоспособность генератора в режиме трансляции питания от электросети восстановлена" in the sql table "журнал_работы_приложения".
    pub fn log_generator_work_restored() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event =
            "Работоспособность генератора в режиме трансляции питания от электросети восстановлена";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Отправлено SMS сообщение: /Авария! Генератор неисправен! Срочно произведите сервисные работы!/ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_generator_work_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Отправлено SMS сообщение: /Авария! Генератор неисправен! Срочно произведите сервисные работы!/ на номер +79139402913";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Отправлено SMS сообщение: /Работоспособность генератора в режиме трансляции питания от электросети восстановлена./ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_generator_work_restored() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event =
            "Отправлено SMS сообщение: /Работоспособность генератора в режиме трансляции питания от электросети восстановлена./ на номер +79139402913";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Server error! Ошибка! SMS уведомление не было отправлено!" in the sql table "журнал_работы_приложения".
    pub fn log_server_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Server error! Ошибка! SMS уведомление не было отправлено!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Http request status error! Ошибка! SMS уведомление не было отправлено!" in the sql table "журнал_работы_приложения".
    pub fn log_request_status_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Http request status error! Ошибка! SMS уведомление не было отправлено!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Ошибка! Доступ к интернету отсутствует! Http запрос не был выполнен! SMS уведомление не было отправлено!" in the sql table "журнал_работы_приложения".
    pub fn log_internet_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Ошибка! Доступ к интернету отсутствует! Http запрос не был выполнен! SMS уведомление не было отправлено!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Ошибка! Связь OPC сервера с ПЛК отсутствует!" in the sql table "журнал_работы_приложения".
    pub fn log_plc_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Ошибка! Связь Modbus клиента с ПЛК отсутствует!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Ошибка! Связь СУБД PostgreSQL с OPC сервером отсутствует!" in the sql table "журнал_работы_приложения".
    pub fn log_opc_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records the event "Питание от электросети есть." in the sql table "события_авр".
    pub fn event_power_ok() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Питание от электросети есть.";
        client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Сбой питания от электросети. Успешный старт генератора." in the sql table "события_авр".
    pub fn event_power_failure_start_generator_ok() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Сбой питания от электросети. Успешный старт генератора.";
        client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Сбой питания от электросети. Сбой старта генератора." in the sql table "события_авр".
    pub fn event_power_failure_start_generator_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Сбой питания от электросети. Сбой старта генератора.";
        client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Питание от электросети восстановлено. Генератор исправен. Генератор работает." in the sql table "события_авр".
    pub fn event_power_restored_generator_work_ok() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Питание от электросети восстановлено. Генератор исправен. Генератор работает.";
        client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Питание от электросети восстановлено. Генератор неисправен. Генератор не работает." in the sql table "события_авр".
    pub fn event_power_restored_generator_work_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event =
            "Питание от электросети восстановлено. Генератор неисправен. Генератор не работает.";
        client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records log "Произошел сбой питания от электросети! Ожидание (90 секунд) подтверждения отсутствия питания от электросети." in the sql table "журнал_работы_приложения".
    pub fn log_power_failure() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Произошел сбой питания от электросети! Ожидание (90 секунд) подтверждения отсутствия питания от электросети.";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Питание от электросети есть." in the sql table "журнал_работы_приложения".
    pub fn log_power_ok() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Питание от электросети есть.";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Подтверждение отсутствия питания от электросети." in the sql table "журнал_работы_приложения".
    pub fn log_power_failure_confirmed() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Подтверждение отсутствия питания от электросети.";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Успешный старт генератора." in the sql table "журнал_работы_приложения".
    pub fn log_start_generator_ok() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Успешный старт генератора.";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Сбой старта генератора!" in the sql table "журнал_работы_приложения".
    pub fn log_start_generator_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Сбой старта генератора!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Питание от электросети восстановлено." in the sql table "журнал_работы_приложения".
    pub fn log_power_restored() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Питание от электросети восстановлено.";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Генератор исправен. Генератор работает." in the sql table "журнал_работы_приложения".
    pub fn log_power_restored_generator_ok() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Генератор исправен. Генератор работает.";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Генератор неисправен. Генератор не работает." in the sql table "журнал_работы_приложения".
    pub fn log_power_restored_generator_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Генератор неисправен. Генератор не работает.";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Питание от электросети еще не было восстановлено, после отключения." in the sql table "журнал_работы_приложения".
    pub fn log_power_dont_restored() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Питание от электросети еще не было восстановлено, после отключения.";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Отправлено SMS сообщение: /Сбой питания от электросети. Успешный старт генератора./ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_start_generator_ok() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Отправлено SMS сообщение: /Сбой питания от электросети. Успешный старт генератора./ на номер +79139402913";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Отправлено SMS сообщение: /Сбой питания от электросети. Сбой старта генератора./ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_start_generator_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Отправлено SMS сообщение: /Сбой питания от электросети. Сбой старта генератора./ на номер +79139402913";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор исправен. Генератор работает./ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_power_restored_generator_ok() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор исправен. Генератор работает./ на номер +79139402913";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор неисправен. Генератор не работает./ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_power_restored_generator_err() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        let event = "Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор неисправен. Генератор не работает./ на номер +79139402913";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Get the time (unix) of the last entry in a table "avr_control_insert" and write it to the db "skydb" in RAM
    pub fn unix_from_sql() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        for row in client.query(
            "SELECT EXTRACT(epoch FROM mark) FROM avr_control_insert ORDER BY mark DESC limit 1",
            &[],
        )? {
            let unix_from_sql = UnixFromSql { time: row.get(0) };
            crate::skydb::skytable::set_f64_skydb("unix_from_sql", &unix_from_sql.time.to_string());
        }
        Ok(())
    }

    /// Get the time (unix) now from PostgreSQL and write it to the db "skydb" in RAM
    pub fn unix_from_sql_now() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        for row in client.query(
            "SELECT EXTRACT(epoch FROM now()) FROM avr_control_insert ORDER BY now() DESC limit 1",
            &[],
        )? {
            let unix_from_sql_now = UnixFromSqlNow { time: row.get(0) };
            crate::skydb::skytable::set_f64_skydb(
                "unix_from_sql_now",
                &unix_from_sql_now.time.to_string(),
            );
        }
        Ok(())
    }

    /// Get latest value of plc_connect from PostgreSQL and write to the db "skydb" in RAM
    pub fn plc_connect() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        for row in client.query(
            "SELECT mark, connection FROM avr_control_insert ORDER BY mark DESC limit 1",
            &[],
        )? {
            let plc_connect = PlcConnect {
                connection: row.get(1),
            };
            crate::skydb::skytable::set_i32_skydb(
                "plc_connect",
                &plc_connect.connection.to_string(),
            );
        }
        Ok(())
    }

    /// Get latest value of generator_faulty from PostgreSQL and write to the db "skydb" in RAM
    pub fn generator_faulty() -> Result<(), Error> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        for row in client.query(
            "SELECT mark, generator_faulty FROM avr_control_insert ORDER BY mark DESC limit 1",
            &[],
        )? {
            let faulty = &Faulty {
                generator_faulty: row.get(1),
            };
            crate::skydb::skytable::set_i32_skydb(
                "generator_faulty",
                &faulty.generator_faulty.to_string(),
            );
        }
        Ok(())
    }
}
