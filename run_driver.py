from selenium import webdriver


if __name__ == '__main__':
    driver = webdriver.Edge()
    driver.get('https://www.google.com/')
    cmd = 'http://localhost:{}/session/{}/screenshot'.format(driver.service.port, driver.session_id)
    print(cmd)
    input('To exit, please enter the enter key')
    