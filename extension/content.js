(async () => {
  const address = chrome.runtime.getURL("pkg/cat_tutor.js");
  const mod = await import(address);
  
  const {default: init, run} = mod; 
  await init();
  run();
})();

