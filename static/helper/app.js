// Tab switching functionality
document.querySelectorAll('.tab').forEach(tab => {
    tab.addEventListener('click', () => {
        document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
        document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
        
        tab.classList.add('active');
        document.getElementById(tab.dataset.tab).classList.add('active');
    });
});

// Algorithm selection
document.querySelectorAll('.crypto-card').forEach(card => {
    card.addEventListener('click', () => {
        const container = card.parentElement;
        container.querySelectorAll('.crypto-card').forEach(c => c.classList.remove('selected'));
        card.classList.add('selected');
    });
});

// File drop zone functionality
function setupDropZone(dropZoneId, fileInputId, fileListId) {
    const dropZone = document.getElementById(dropZoneId);
    const fileInput = document.getElementById(fileInputId);
    const fileList = document.getElementById(fileListId);

    dropZone.addEventListener('dragover', (e) => {
        e.preventDefault();
        dropZone.classList.add('dragover');
    });

    dropZone.addEventListener('dragleave', () => {
        dropZone.classList.remove('dragover');
    });

    dropZone.addEventListener('drop', (e) => {
        e.preventDefault();
        dropZone.classList.remove('dragover');
        const files = e.dataTransfer.files;
        handleFiles(files, fileListId);
    });

    fileInput.addEventListener('change', (e) => {
        handleFiles(e.target.files, fileListId);
    });
}


function handleFiles(files, fileListId) {
    const fileList = document.getElementById(fileListId);
    fileList.innerHTML = '';

    Array.from(files).forEach(file => {
        const fileItem = document.createElement('div');
        fileItem.className = 'file-item';
        
        const isLarge = file.size > 50 * 1024 * 1024; // 50MB threshold for streaming
        
        fileItem.innerHTML = `
            <div class="file-info">
                <div class="file-name">${file.name}</div>
                <div class="file-size">${formatFileSize(file.size)} ${isLarge ? '(Streaming enabled)' : ''}</div>
            </div>
            <div class="file-status status-pending">Pending</div>
        `;
        
        fileList.appendChild(fileItem);
    });

    // Show streaming indicator for large files
    const hasLargeFiles = Array.from(files).some(file => file.size > 50 * 1024 * 1024);
    const streamingIndicator = document.querySelector('.streaming-indicator');
    if (streamingIndicator) {
        streamingIndicator.style.display = hasLargeFiles ? 'inline-flex' : 'none';
    }
}

function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}


// Setup all drop zones
setupDropZone('encryptDropZone', 'encryptFiles', 'encryptFileList');
setupDropZone('decryptDropZone', 'decryptFiles', 'decryptFileList');
setupDropZone('batchDropZone', 'batchFiles', 'batchFileList');

// Form submission handlers
document.getElementById('encryptForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const password = document.getElementById('encryptPassword').value;
    const confirmPassword = document.getElementById('encryptConfirm').value;
    
    if (password !== confirmPassword) {
        alert('Passwords do not match!');
        return;
    }
    
    const files = document.getElementById('encryptFiles').files;
    const algorithm = document.querySelector('.crypto-card.selected').dataset.algo;
    console.log(algorithm)
    console.log(password)
     const formData = new FormData();
      formData.append("algorithm", algorithm);
      formData.append("password", password);
      Array.from(files).forEach(file => {
        formData.append('files', file);
      });
      await process('encrypt');
        const response = await fetch('/api/encrypt', {
            method: 'POST',
            body: formData
        });
        const data = await response.json(); // parses JSON
         if(data){
            console.log("process_complete");
            console.log(data); 
            await process_complete('encrypt');
            const downloadSection = document.getElementById('downloadSection');
            const downloadLinks = document.getElementById('downloadLinks');
            
            downloadLinks.innerHTML = '';
            if(data.result){
                console.log(data.result);
                data.result.forEach( item =>{
                   
                        const downloadBtn = document.createElement('button');
                        downloadBtn.className = 'btn download-btn';
                        downloadBtn.textContent = `📥 ${item[1]}`;
                        downloadBtn.onclick = () => downloadBase64AsFile(item[0],item[1]);
                        downloadLinks.appendChild(downloadBtn);

                   
                } )
            }
             downloadSection.style.display = 'block';
        }
        console.log(" succcess:", data.success);
        console.log(" message:", data.message);
});

function downloadBase64AsFile(base64Data, filename) {
    const byteCharacters = atob(base64Data);
    const byteNumbers = new Array(byteCharacters.length);
    for (let i = 0; i < byteCharacters.length; i++) {
        byteNumbers[i] = byteCharacters.charCodeAt(i);
    }
    const byteArray = new Uint8Array(byteNumbers);
    const blob = new Blob([byteArray], { type: 'application/octet-stream' });
    const link = document.createElement('a');
    link.href = URL.createObjectURL(blob);
    link.download = filename.endsWith('.enc') ? filename : filename + '.enc';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
}

document.getElementById('decryptForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const password = document.getElementById('decryptPassword').value;
    const files = document.getElementById('decryptFiles').files;

    console.log(password)
    

    const formData = new FormData();
    formData.append("password", password);
    for (const file of files) {
        const base64String = await new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = () => {
                const base64 = reader.result.split(',')[1]; // Strip "data:*/*;base64,"
                resolve(base64);
            };
            reader.onerror = reject;
            reader.readAsDataURL(file);
        });

        formData.append("file", base64String); // Important: use "file"
    }
    console.log(files.length);
    
    await process('decrypt')
        const response = await fetch('/api/decrypt', {
            method: 'POST',
            body: formData
        });
   
    const data = await response.json()
    console.log(data)
    if(data){
        console.log("process_complete");
        process_complete('decrypt');
        const downloadSection = document.getElementById('downloadSection');
        const downloadLinks = document.getElementById('downloadLinks');
        
        downloadLinks.innerHTML = '';
        data.decrypted_data.forEach((item,index)=>{
            const downloadBtn = document.createElement('button');
            downloadBtn.className = 'btn download-btn';
            downloadBtn.textContent = `📥 Decrypted File ${index+1}`;
            downloadBtn.onclick = () => downloadPdf(item);
            downloadLinks.appendChild(downloadBtn);
        })
        
        
        downloadSection.style.display = 'block';

    }
    console.log(data);
   
});

// Real-time password matching
document.getElementById('encryptConfirm').addEventListener('input', (e) => {
    const password = document.getElementById('encryptPassword').value;
    const confirm = e.target.value;
    
    if (confirm && password !== confirm) {
        e.target.style.borderColor = '#ef4444';
    } else {
        e.target.style.borderColor = '#e5e7eb';
    }
});


document.getElementById('batchForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    const password = document.getElementById('batchPassword').value;
    const files = document.getElementById('batchFiles').files;
    const algorithm = document.getElementById('batchAlgorithm').value;
    const operation = document.querySelector('#batch .crypto-card.selected').dataset.operation;
   
    if(operation=="encrypt"){
         console.log(operation)
        const formData = new FormData();
        formData.append('password', password);
        formData.append('algorithm', algorithm);
        
        Array.from(files).forEach(file => {
            formData.append('files', file);
        });
        console.log(files)
        await process('batch');
        console.log(1);
        const response = await fetch('/api/batch_encrypt', {
            method: 'POST',
            body: formData
        });
        const data = await response.json();
         if(data){
            console.log("process_complete");
            console.log(data); 
            await process_complete('batch');
            const downloadSection = document.getElementById('downloadSection');
            const downloadLinks = document.getElementById('downloadLinks');
            
            downloadLinks.innerHTML = '';
            
                const downloadBtn = document.createElement('button');
                downloadBtn.className = 'btn download-btn';
                downloadBtn.textContent = `📥 ${data.file_id}`;
                downloadBtn.onclick = () => downloadBase64AsFile(data.encrypted_data,data.file_id);
                downloadLinks.appendChild(downloadBtn);

            downloadSection.style.display = 'block';

        }
          //downloadBase64AsFile(data.encrypted_data,data.file_id);
        console.log(" succcess:", data.success);
        console.log(" message:", data.message);
        console.log(" File ID:", data.file_id);
        console.log(" Encrypted Data (Base64):", data.encrypted_data);
    }
    else{
       
        console.log(password)
        const file = files[0];

        const base64String = await new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = () => {
                const base64 = reader.result.split(',')[1]; // Strip "data:*/*;base64,"
                resolve(base64);
            };
            reader.onerror = reject;
            reader.readAsDataURL(file);
        });

        const formData = new FormData();
        formData.append("password", password);
        formData.append("files", base64String); 
        await process('batch')
        const response = await fetch('/api/batch_decrypt', {
            method: 'POST',
            body: formData
        });
        const data = await response.json();
        if(data){
            console.log("process_complete");
            process_complete('batch');
            const downloadSection = document.getElementById('downloadSection');
            const downloadLinks = document.getElementById('downloadLinks');
            
            downloadLinks.innerHTML = '';
            
            if (data.files) {
                data.files.forEach((file,index )=> {
                    const downloadBtn = document.createElement('button');
                    downloadBtn.className = 'btn download-btn';
                    downloadBtn.textContent = `📥 Decrypted File-${index+1}`;
                    downloadBtn.onclick = () => downloadPdf(file);
                    downloadLinks.appendChild(downloadBtn);
                });
            }
            
            downloadSection.style.display = 'block';

        }
        console.log(data);
        // for(k=0;k<data.files.length;k++){
        //     const binaryString = atob(data.files[k]);
        //     const len = binaryString.length;
        //     const bytes = new Uint8Array(len);

        //     for (let i = 0; i < len; i++) {
        //         bytes[i] = binaryString.charCodeAt(i);
        //     }

        //     const blob = new Blob([bytes], { type: "application/pdf" }); // Set proper MIME
        //     const url = URL.createObjectURL(blob);

        //     const a = document.createElement("a");
        //     a.href = url;
        //     a.download ="decrypted.pdf";  // fallback filename
        //     document.body.appendChild(a);
        //     a.click();
        //     document.body.removeChild(a);
        //     URL.revokeObjectURL(url);
        // }

    }
    
});

async function downloadPdf(file) {
    const binaryString = atob(file);
    const len = binaryString.length;
    const bytes = new Uint8Array(len);

    for (let i = 0; i < len; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }

    const blob = new Blob([bytes], { type: "application/pdf" }); // Set proper MIME
    const url = URL.createObjectURL(blob);

    const a = document.createElement("a");
    a.href = url;
    a.download ="decrypted.pdf";  // fallback filename
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
}
const progressIntervals = {};  
async function process(operation) {
    const progressContainer = document.getElementById(`${operation}Progress`);
    const progressFill = progressContainer.querySelector('.progress-fill');
    const progressText = progressContainer.querySelector('.progress-text');
    const submitBtn = document.getElementById(`${operation}Btn`);
    
    progressContainer.style.display = 'block';
    submitBtn.disabled = true;
    submitBtn.textContent = 'Processing...';
    submitBtn.classList.add('processing');
    
    const fileItems = document.querySelectorAll(`#${operation}FileList .file-item`);
    
        let progress = 0;
        const interval = setInterval(() => {
            progress += Math.random() * 15;
            if (progress > 90) progress = 90;
            
            progressFill.style.width = progress + '%';
            progressText.textContent = `Processing... ${Math.round(progress)}%`;
            
            // Update file statuses
            fileItems.forEach((item, index) => {
                const status = item.querySelector('.file-status');
                if (progress >= (index + 1) * (90 / fileItems.length)) {
                    status.textContent = 'Processing';
                    status.className = 'file-status status-processing';
                }
            });
            
            // if (progress >= 90) {
            //     clearInterval(progressInterval);
            // }
        }, 200);
        progressIntervals[operation] = interval;
        
}
async function process_complete(operation) {
    console.log(1);
    const progressContainer = document.getElementById(`${operation}Progress`);
    const progressFill = progressContainer.querySelector('.progress-fill');
    const progressText = progressContainer.querySelector('.progress-text');
    const submitBtn = document.getElementById(`${operation}Btn`);
    clearInterval(progressIntervals[operation])
    progressFill.style.width = '100%';
    progressText.textContent = 'Complete!';
    const fileItems = document.querySelectorAll(`#${operation}FileList .file-item`);

    // Update all file statuses to complete
    fileItems.forEach(item => {
        const status = item.querySelector('.file-status');
        status.textContent = 'Complete';
        status.className = 'file-status status-complete';
    });
    
    submitBtn.disabled = false;
    submitBtn.textContent = operation === 'encrypt' ? '🔒 Encrypt Files' : 
                            operation === 'decrypt' ? '🔓 Decrypt Files' : '📦 Process Batch';
    submitBtn.classList.remove('processing');
}
async function showDownloadSection(data) {
    const downloadSection = document.getElementById('downloadSection');
    const downloadLinks = document.getElementById('downloadLinks');
    
    downloadLinks.innerHTML = '';
    
    if (data.files) {
        data.files.forEach(file => {
            const downloadBtn = document.createElement('button');
            downloadBtn.className = 'btn download-btn';
            downloadBtn.textContent = `📥 ${file.name}`;
            downloadBtn.onclick = () => downloadFile(file.url, file.name);
            downloadLinks.appendChild(downloadBtn);
        });
    }
    
    downloadSection.style.display = 'block';
}
