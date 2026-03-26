use sqlx::SqlitePool;

// ==========================================
// DATABASE SEEDER
// ==========================================

/// Populates the database with initial mock data.
pub async fn seed_database(pool: &SqlitePool) {
    let seeder_sql = r#"
    -- 1. Teachers
    INSERT OR IGNORE INTO teachers (id, name, email) VALUES 
    (1, 'Carlos Mondlane', 'carlos@escola.mz'),
    (2, 'Maria Santos', 'maria@escola.mz'),
    (3, 'João Silva', 'joao@escola.mz');

    -- 2. Classes
    INSERT OR IGNORE INTO classes (id, name) VALUES 
    (1, '10ª Classe A'),
    (2, '10ª Classe B'),
    (3, '12ª Classe A');

    -- 3. Subjects
    INSERT OR IGNORE INTO subjects (id, name) VALUES 
    (1, 'Matemática'),
    (2, 'Biologia'),
    (3, 'Física');

    -- 4. Students
    INSERT OR IGNORE INTO students (id, student_code, name, class_id) VALUES 
    (1, '06.0842.2024', 'Ana Langa', 1),
    (2, '06.0843.2024', 'Pedro Macamo', 1),
    (3, '06.0844.2024', 'Sofia Tembe', 1),
    (4, '06.0845.2024', 'Lucas Sitoe', 1),
    (5, '06.0846.2024', 'Inês Cossa', 1),
    (6, '06.0847.2024', 'Tiago Matusse', 1),
    (7, '06.0848.2024', 'Marcos Nhaca', 1),
    (8, '06.0849.2024', 'Marta Cumbane', 1),
    (9, '06.0850.2024', 'José Maposse', 1),
    (10, '06.0851.2024', 'Luísa Nhampossa', 1),
    (11, '06.0852.2024', 'Filipe Nyusi', 2),
    (12, '06.0853.2024', 'Graça Machel', 2),
    (13, '06.0854.2024', 'Eduardo Mondlane', 3),
    (14, '06.0855.2024', 'Samora Machel', 3);

    -- 5. Sessions
    INSERT OR IGNORE INTO sessions (id, title, teacher_id, class_id, subject_id, status, time_limit_seconds) VALUES 
    (1, 'Avaliação Final de Matemática', 1, 1, 1, 'completed', 600),
    (2, 'Teste Contínuo de Biologia', 2, 1, 2, 'active', 600),
    (3, 'Rascunho: Exercícios de Física', 3, 2, 3, 'draft', 900);

    -- 6. Questions
    INSERT OR IGNORE INTO questions (id, session_id, text) VALUES 
    (1, 1, 'Quanto é 7 x 8?'),
    (2, 1, 'Qual é a raiz quadrada de 144?'),
    (3, 1, 'Se x + 5 = 12, qual é o valor de x?'),
    (4, 1, 'Quanto é 15% de 200?'),
    (5, 1, 'Qual é o valor aproximado de Pi (π)?'),
    (6, 1, 'Quanto é 2 elevado a 4 (2^4)?'),
    (7, 2, 'Qual é o processo pelo qual as plantas produzem seu próprio alimento?'),
    (8, 2, 'Qual é o maior órgão do corpo humano?'),
    (9, 2, 'Como se chama o pigmento que dá a cor verde às plantas?'),
    (10, 2, 'Qual é o único mamífero capaz de voar?'),
    (11, 2, 'Que gás nós expiramos na respiração?'),
    (12, 2, 'Onde o DNA está localizado na célula eucariótica?'),
    (13, 3, 'Qual é a unidade de medida da Força no Sistema Internacional?'),
    (14, 3, 'Qual é a velocidade da luz no vácuo (aproximadamente)?'),
    (15, 3, 'Quem formulou as três leis do movimento?'),
    (16, 3, 'Qual é a fórmula da energia cinética?'),
    (17, 3, 'O que mede um termômetro?'),
    (18, 3, 'Qual estado da matéria tem forma e volume definidos?');

    -- 7. Options
    INSERT OR IGNORE INTO options (id, question_id, text, is_correct) VALUES 
    (1, 1, '54', 0), (2, 1, '56', 1), (3, 1, '64', 0), (4, 1, '48', 0),
    (5, 2, '10', 0), (6, 2, '14', 0), (7, 2, '12', 1), (8, 2, '16', 0),
    (9, 3, '5', 0), (10, 3, '7', 1), (11, 3, '17', 0), (12, 3, '8', 0),
    (13, 4, '20', 0), (14, 4, '40', 0), (15, 4, '30', 1), (16, 4, '50', 0),
    (17, 5, '3.14', 1), (18, 5, '3.41', 0), (19, 5, '2.14', 0), (20, 5, '4.13', 0),
    (21, 6, '8', 0), (22, 6, '16', 1), (23, 6, '32', 0), (24, 6, '64', 0),
    (25, 7, 'Respiração', 0), (26, 7, 'Digestão', 0), (27, 7, 'Fotossíntese', 1), (28, 7, 'Fermentação', 0),
    (29, 8, 'Fígado', 0), (30, 8, 'Pele', 1), (31, 8, 'Cérebro', 0), (32, 8, 'Coração', 0),
    (33, 9, 'Melanina', 0), (34, 9, 'Clorofila', 1), (35, 9, 'Caroteno', 0), (36, 9, 'Hemoglobina', 0),
    (37, 10, 'Morcego', 1), (38, 10, 'Esquilo-voador', 0), (39, 10, 'Avestruz', 0), (40, 10, 'Pinguim', 0),
    (41, 11, 'Oxigênio', 0), (42, 11, 'Nitrogênio', 0), (43, 11, 'Dióxido de Carbono', 1), (44, 11, 'Hélio', 0),
    (45, 12, 'No citoplasma', 0), (46, 12, 'No ribossomo', 0), (47, 12, 'No núcleo', 1), (48, 12, 'Na membrana', 0),
    (49, 13, 'Joule', 0), (50, 13, 'Newton', 1), (51, 13, 'Watt', 0), (52, 13, 'Pascal', 0),
    (53, 14, '300.000 km/s', 1), (54, 14, '150.000 km/s', 0), (55, 14, '1.000.000 km/s', 0), (56, 14, '340 m/s', 0),
    (57, 15, 'Albert Einstein', 0), (58, 15, 'Galileu Galilei', 0), (59, 15, 'Isaac Newton', 1), (60, 15, 'Nikola Tesla', 0),
    (61, 16, 'E = mc^2', 0), (62, 16, 'F = m*a', 0), (63, 16, 'E = m*g*h', 0), (64, 16, 'E = (m*v^2)/2', 1),
    (65, 17, 'Pressão', 0), (66, 17, 'Temperatura', 1), (67, 17, 'Umidade', 0), (68, 17, 'Densidade', 0),
    (69, 18, 'Líquido', 0), (70, 18, 'Gasoso', 0), (71, 18, 'Plasma', 0), (72, 18, 'Sólido', 1);

    -- 8. Scores 
    INSERT OR IGNORE INTO scores (id, session_id, student_id, score, played_at, synced_at, local_attempt_id) VALUES 
    (1, 1, 1, 6, '2026-03-24 10:00:00', '2026-03-24 10:15:00', 'uuid-attempt-001'),
    (2, 1, 2, 5, '2026-03-24 10:02:00', '2026-03-24 10:15:00', 'uuid-attempt-002'),
    (3, 1, 3, 4, '2026-03-24 10:05:00', '2026-03-24 10:15:00', 'uuid-attempt-003'),
    (4, 1, 4, 6, '2026-03-24 10:06:00', '2026-03-24 10:15:00', 'uuid-attempt-004'),
    (5, 1, 5, 3, '2026-03-24 10:10:00', '2026-03-24 10:15:00', 'uuid-attempt-005'),
    (6, 1, 6, 5, '2026-03-24 10:11:00', '2026-03-24 10:15:00', 'uuid-attempt-006'),
    (7, 1, 7, 2, '2026-03-24 10:12:00', '2026-03-24 10:15:00', 'uuid-attempt-007'),
    (8, 1, 8, 6, '2026-03-24 10:14:00', '2026-03-24 10:15:00', 'uuid-attempt-008');
    "#;

    sqlx::query(seeder_sql)
        .execute(pool)
        .await
        .expect("Failed to execute database seeder");

    println!("🌱 Database successfully seeded with mock data!");
}
