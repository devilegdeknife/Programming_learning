import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import cv2
from deepface import DeepFace
import threading

# 定义一个面部检测应用程序类
class FaceDetectionApp:
    def __init__(self, root):
        self.live_detection_running = None  # 实时检测线程的运行状态
        self.root = root  # Tkinter的根窗口
        self.root.title("Face Detection App")  # 设置窗口标题
        self.main_menu_window = None  # 主菜单窗口
        self.cap = cv2.VideoCapture(0)  # 打开默认摄像头
        self.last_frame = None  # 存储最后一帧图像
        self.create_login_form()  # 创建登录表单

    # 创建登录表单
    def create_login_form(self):
        self.root.geometry("300x150")  # 设置窗口大小
        self.root.resizable(False, False)  # 禁止调整窗口大小

        # 设置标签的背景颜色和前景颜色
        style = ttk.Style(self.root)
        style.configure("TLabel", background="#333333")
        style.configure("TLabel", foreground="white")

        # 创建用户名和密码的标签和输入框
        ttk.Label(self.root, text="Username:", style="TLabel").grid(row=0, column=0, sticky="e", padx=5, pady=5)
        self.entry_username = ttk.Entry(self.root)
        self.entry_username.grid(row=0, column=1, sticky="ew", padx=5, pady=5)

        ttk.Label(self.root, text="Password:", style="TLabel").grid(row=1, column=0, sticky="e", padx=5, pady=5)
        self.entry_password = ttk.Entry(self.root, show="*")
        self.entry_password.grid(row=1, column=1, sticky="ew", padx=5, pady=5)

        # 创建登录按钮，点击时调用login函数
        ttk.Button(self.root, text="Login", command=self.login).grid(row=2, column=0, columnspan=2, pady=10)

    # 处理登录操作
    def login(self):
        username = self.entry_username.get()  # 获取用户名
        password = self.entry_password.get()  # 获取密码

        # 检查用户名和密码是否正确
        if username == "admin" and password == "password":
            messagebox.showinfo("Login Success", "Welcome back, " + username)  # 显示登录成功消息
            self.show_main_menu()  # 显示主菜单
            self.root.withdraw()  # 隐藏登录窗口
        else:
            messagebox.showerror("Login Failed", "Invalid username or password")  # 显示登录失败消息

    # 显示主菜单
    def show_main_menu(self):
        if self.main_menu_window is None or not self.main_menu_window.winfo_exists():
            self.main_menu_window = tk.Toplevel(self.root)  # 创建一个新的顶级窗口
            self.main_menu_window.title("Main Menu")  # 设置窗口标题
            self.main_menu_window.geometry('400x300')  # 设置窗口大小
            self.main_menu_window.protocol("WM_DELETE_WINDOW", self.on_close)  # 设置窗口关闭事件处理函数

            # 设置窗口的背景颜色和文字颜色
            self.main_menu_window.configure(bg="#333333")
            self.display_area = ttk.Label(self.main_menu_window, text="Function Display Area", background='#333333', foreground="white")
            self.display_area.place(relx=0.5, rely=0.3, anchor='center')

            # 创建实时面部情绪检测按钮和退出程序按钮
            self.button_live = ttk.Button(self.main_menu_window, text="Live Face Emotion Detection", command=self.detect_faces_live)
            self.button_live.place(relx=0.5, rely=0.5, anchor='center')

            self.button_exit = ttk.Button(self.main_menu_window, text="Exit Program", command=self.exit_program)
            self.button_exit.place(relx=0.5, rely=0.7, anchor='center')

            self.main_menu_window.minsize(300, 200)  # 设置窗口的最小大小

    # 退出程序
    def exit_program(self):
        if self.main_menu_window:
            self.main_menu_window.destroy()  # 销毁主菜单窗口
        self.cap.release()  # 释放摄像头
        cv2.destroyAllWindows()  # 销毁所有OpenCV窗口
        self.root.destroy()  # 销毁根窗口

    # 处理窗口关闭事件
    def on_close(self):
        if self.main_menu_window:
            self.main_menu_window.destroy()  # 销毁主菜单窗口
        else:
            self.root.destroy()  # 销毁根窗口

    # 实时检测面部情绪
    def detect_faces_live(self):
        if not self.cap.isOpened():
            messagebox.showerror("Error", "Unable to open the camera")  # 显示错误消息
            return

        self.live_detection_running = threading.Event()  # 创建一个线程事件对象
        self.live_detection_running.set()  # 设置线程事件

        # 定义实时检测函数
        def live_detection():
            while self.live_detection_running.is_set() and self.cap.isOpened():
                ret, frame = self.cap.read()
                if ret:
                    results = DeepFace.analyze(frame, actions=['age', 'emotion', 'gender', 'race'],
                                               detector_backend="mtcnn", enforce_detection=False)
                    for result in results:
                        age = result["age"]
                        emotion = result["dominant_emotion"]
                        gender = result["dominant_gender"]
                        race = result["dominant_race"]
                        region = result["region"]

                        cv2.rectangle(
                            frame, (region["x"], region["y"]),
                            (region["x"] + region["w"], region["y"] + region["h"]),
                            (255, 0, 0), 1
                        )
                        cv2.putText(frame,
                                    f"age: {age}, emotion: {emotion}, gender: {gender}, race: {race}",
                                    (region["x"] - 135, region["y"]),
                                    cv2.FONT_HERSHEY_SIMPLEX,
                                    0.6,
                                    (161, 16, 203),
                                    1,
                                    cv2.LINE_AA)

                        cv2.imshow('Emotion Detection', frame)

                        # 检查窗口是否仍然存在
                        if cv2.getWindowProperty('Emotion Detection', cv2.WND_PROP_VISIBLE) < 1:
                            self.live_detection_running = False
                            break

                        if cv2.waitKey(1) & 0xFF == ord(' '):
                            self.live_detection_running = False
                            break

                    self.last_frame = frame
                    cv2.imwrite("last_frame.jpg", frame)

            # 释放摄像头并销毁窗口
            self.cap.release()
            cv2.destroyAllWindows()
            self.show_main_menu()

        # 创建并启动实时检测线程
        self.live_detection_thread = threading.Thread(target=live_detection)
        self.live_detection_thread.start()

        # 重写窗口关闭事件处理函数
        self.main_menu_window.protocol("WM_DELETE_WINDOW", self.on_close_window)

    # 处理窗口关闭事件
    def on_close_window(self):
        self.stop_all_threads()  # 停止所有线程
        self.main_menu_window.destroy()  # 销毁主菜单窗口

    # 停止所有线程
    def stop_all_threads(self):
        # 停止视频捕获线程
        self.live_detection_running.clear()
        if self.live_detection_thread.is_alive():
            self.live_detection_thread.join()

        # 释放视频捕获设备
        self.cap.release()

        # 销毁所有OpenCV窗口
        cv2.destroyAllWindows()

        # 停止可能存在的其他线程
        # 例如，如果有其他后台任务线程，也应该在这里停止它们

    # def detect_faces_image(self):
    #     if self.main_menu_window:
    #         self.main_menu_window.destroy()
    #
    #     file_path = filedialog.askopenfilename()
    #     if not file_path:
    #         print("没有选择文件。")
    #         self.show_main_menu()
    #         return
    #
    #     try:
    #         result = DeepFace.analyze(file_path, actions=['emotion'], enforce_detection=False)
    #     except ValueError as e:
    #         print("错误:", e)
    #         self.show_main_menu()
    #         return
    #
    #     if isinstance(result, dict) and 'dominant_emotion' in result:
    #         print(result['dominant_emotion'])
    #         try:
    #             img = cv2.imread(file_path)
    #             cv2.putText(img, result['dominant_emotion'], (50, 50), cv2.FONT_HERSHEY_SIMPLEX, 1,
    #                         (0, 0, 255), 2, cv2.LINE_4)
    #             cv2.imshow('情绪检测', img)
    #             cv2.waitKey(0)
    #             cv2.destroyAllWindows()
    #         except Exception as e:
    #             print("显示图片时出错:", e)
    #
    #     self.show_main_menu()


# 创建一个Tkinter应用程序
if __name__ == "__main__":
    root = tk.Tk()
    style = ttk.Style(root)
    style.theme_use("clam")  # 使用clam主题以获得Material Design风格
    app = FaceDetectionApp(root)  # 创建一个面部检测应用程序对象
    root.mainloop()  # 启动Tkinter应用程序
