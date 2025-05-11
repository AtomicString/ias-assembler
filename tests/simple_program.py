from selenium import webdriver
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.by import By
from time import sleep

driver = webdriver.Firefox()

driver.get("http://127.0.0.1:8000/")

editor = driver.find_element(By.ID, "editor")
next_btn = driver.find_element(By.ID, "next")
compile_btn = driver.find_element(By.ID, "compile")
mem_contents = driver.find_element(By.ID, "mem-row")

pc_text = driver.find_element(By.CSS_SELECTOR, "#PC>.val")
ac_text = driver.find_element(By.CSS_SELECTOR, "#AC>.val")
mq_text = driver.find_element(By.CSS_SELECTOR, "#MQ>.val")

editor.send_keys("LOAD M(300)\nADD M(301)\nSTOR M(302)\n")
compile_btn.click()

sleep(0.5)
mem_300 = mem_contents.find_element(By.CSS_SELECTOR, "*:nth-child(301)>input")
mem_301 = mem_contents.find_element(By.CSS_SELECTOR, "*:nth-child(302)>input")

driver.execute_script("arguments[0].scrollIntoView({ behavior: 'smooth', block: 'center' });", mem_300)
sleep(0.5)

mem_300.click()
mem_300.clear()
mem_300.send_keys("10" + Keys.RETURN)
mem_301.click()
mem_301.clear()
mem_301.send_keys("2" + Keys.RETURN)

next_btn.click()

assert pc_text.text == "1"
assert ac_text.text == "16"

next_btn.click()

assert pc_text.text == "2"
assert ac_text.text == "18"

next_btn.click()

assert pc_text.text == "2"
mem_302 = mem_contents.find_element(By.CSS_SELECTOR, "*:nth-child(303)>input")
assert mem_302.get_attribute("value") == "12"

driver.quit()
