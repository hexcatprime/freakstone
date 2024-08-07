import { run } from './loadCsv.js';

document.addEventListener('DOMContentLoaded', () => {
    const manageDataModal = document.getElementById('manageDataModal');
    const manageDataBtn = document.getElementById('manage-data');
    const closeManageDataModal = document.getElementById('closeManageDataModal');
    const saveChangesBtn = document.getElementById('saveChangesBtn');
    const csvTable = document.getElementById('csvTable');

    // Open the manage data modal
    manageDataBtn.addEventListener('click', () => {
        manageDataModal.style.display = 'block';
        loadCsvData();
    });

    // Close the manage data modal
    closeManageDataModal.addEventListener('click', () => {
        manageDataModal.style.display = 'none';
    });

    // Save button
    saveChangesBtn.addEventListener('click', async () => {
        const rows = csvTable.querySelectorAll('tr');
        const data = [];

        // Data
        rows.forEach(row => {
            const cells = row.querySelectorAll('td');
            if (cells.length > 0) {
                const rowData = {};
                cells.forEach((cell, index) => {
                    rowData[`column${index}`] = cell.textContent;
                });
                data.push(rowData);
            }
        });

        // Save CSV
        try {
            const response = await fetch('/save-csv-data', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(data)
            });

            if (response.ok) {
                alert('Changes saved successfully.');
                manageDataModal.style.display = 'none';
                await run();
            } else {
                alert('Failed to save changes.');
            }
        } catch (error) {
            console.error('Error saving changes:', error);
            alert('Error saving changes.');
        }
    });

    // Load CSV data
    async function loadCsvData() {
        try {
            const response = await fetch('/get-csv-data');
            const data = await response.json();
            
            csvTable.innerHTML = ''; // Clear previous data

            if (data.length > 0) {
                const headerRow = document.createElement('tr');
                Object.keys(data[0]).forEach(key => {
                    const th = document.createElement('th');
                    th.textContent = key;
                    headerRow.appendChild(th);
                });
                csvTable.appendChild(headerRow);

                data.forEach(rowData => {
                    const row = document.createElement('tr');
                    Object.values(rowData).forEach(value => {
                        const td = document.createElement('td');
                        td.contentEditable = true; // Make cells editable
                        td.textContent = value;
                        row.appendChild(td);
                    });
                    csvTable.appendChild(row);
                });
            }
        } catch (error) {
            console.error('Error loading CSV data:', error);
        }
    }
});
