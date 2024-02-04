from selenium import webdriver
import pyperclip

if __name__ == '__main__':
    driver = webdriver.Edge()
    driver.get('https://www.google.com/')

    cmd = f'cargo run -- -p {driver.service.port} -s {driver.session_id}'
    pyperclip.copy(cmd)
    print(f'\n{cmd}\n')
    
    input('To exit, please enter the enter key')