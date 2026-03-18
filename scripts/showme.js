// ?showme mode: animated cursor tour of the emulator UI
// Activated by ?showme in the URL query string
(function() {
    if (!window.location.search.includes('showme')) return;

    // Wait for app to render
    setTimeout(startShowMe, 2000);

    function startShowMe() {
        // Create cursor overlay
        const cursor = document.createElement('div');
        cursor.id = 'showme-cursor';
        cursor.innerHTML = '👆';
        cursor.style.cssText = `
            position: fixed; z-index: 99999; font-size: 32px;
            pointer-events: none; transition: all 0.6s ease-in-out;
            top: 50%; left: 50%; transform: translate(-50%, -50%);
            filter: drop-shadow(0 0 8px rgba(255,255,0,0.8));
        `;
        document.body.appendChild(cursor);

        // Create label overlay
        const label = document.createElement('div');
        label.id = 'showme-label';
        label.style.cssText = `
            position: fixed; z-index: 99998; font-size: 18px;
            color: #ffd700; background: rgba(0,0,0,0.8);
            padding: 6px 14px; border-radius: 6px;
            pointer-events: none; opacity: 0;
            transition: opacity 0.3s ease-in-out;
            top: 10px; left: 50%; transform: translateX(-50%);
            font-family: sans-serif; font-weight: bold;
        `;
        document.body.appendChild(label);

        const steps = [
            { desc: 'Click Assembler tab', sel: '.tab-bar .tab:first-child', delay: 1500 },
            { desc: 'Open Examples', sel: '.editor-toolbar .toolbar-btn:first-child', delay: 1500 },
            { desc: 'Select Blink LED', sel: '.example-item:nth-child(3)', delay: 2000 },
            { desc: 'Click Assemble', sel: '#assembleBtn', delay: 2000 },
            { desc: 'Click Run', sel: '.run-btn', delay: 2000 },
            { desc: 'Watch the LED blink...', sel: null, delay: 4000 },
            { desc: 'Click Stop', sel: '.stop-btn', delay: 1500 },
            { desc: 'Expand Instruction Trace', sel: '.trace-header', delay: 2000 },
            { desc: 'Click Step to single-step', sel: '.step-btn', delay: 1500 },
            { desc: 'Step again', sel: '.step-btn', delay: 1500 },
            { desc: 'Step again', sel: '.step-btn', delay: 1500 },
            { desc: 'Tour complete!', sel: null, delay: 3000 },
        ];

        runSteps(cursor, label, steps, 0);
    }

    function runSteps(cursor, label, steps, idx) {
        if (idx >= steps.length) {
            cursor.remove();
            label.remove();
            return;
        }

        const step = steps[idx];
        label.textContent = step.desc;
        label.style.opacity = '1';

        if (step.sel) {
            const el = document.querySelector(step.sel);
            if (el) {
                const rect = el.getBoundingClientRect();
                const cx = rect.left + rect.width / 2;
                const cy = rect.top + rect.height / 2;
                cursor.style.top = cy + 'px';
                cursor.style.left = cx + 'px';

                // Click after cursor arrives
                setTimeout(() => {
                    cursor.style.transform = 'translate(-50%, -50%) scale(0.8)';
                    setTimeout(() => {
                        cursor.style.transform = 'translate(-50%, -50%) scale(1)';
                        el.click();
                    }, 150);
                }, 700);
            }
        }

        setTimeout(() => runSteps(cursor, label, steps, idx + 1), step.delay);
    }
})();
